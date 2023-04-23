use std::time::Duration;

use smithay::{
    backend::{
        renderer::{
            damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement,
            gles::GlesRenderer,
        },
        winit::{self, WinitError, WinitEvent, WinitEventLoop, WinitGraphicsBackend},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{
        calloop::{
            timer::{TimeoutAction, Timer},
            EventLoop,
        },
        wayland_server::Display,
    },
    utils::{Rectangle, Transform},
};

use crate::state::{NoWayData, NoWayState};

#[derive(Debug)]
struct WinitBackendData;

pub fn init_winit() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop = EventLoop::try_new()?;
    let mut display = Display::new()?;

    let state = NoWayState::new(
        event_loop.handle(),
        event_loop.get_signal(),
        &mut display,
        WinitBackendData,
    )?;

    let (mut backend, mut winit) = winit::init::<GlesRenderer>()?;

    let window_size = backend.window_size().physical_size;
    let mode = Mode {
        size: window_size,
        refresh: 60_000,
    };

    let output = Output::new(
        "NoWay".into(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Smithay".into(),
            model: "NoWay".into(),
        },
    );

    let _global = output.create_global::<NoWayState<WinitBackendData>>(&display.handle());
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);

    let mut damage_tracked_renderer = OutputDamageTracker::from_output(&output);
    let mut full_redraw = 0u8;

    let timer = Timer::immediate();
    event_loop
        .handle()
        .insert_source(timer, move |_, _, data| {
            winit_dispatch(
                &mut backend,
                &mut winit,
                data,
                &output,
                &mut damage_tracked_renderer,
                &mut full_redraw,
            )
            .unwrap();
            TimeoutAction::ToDuration(Duration::from_millis(16))
        })?;

    let mut data = NoWayData { state, display };
    event_loop.run(None, &mut data, move |_| {})?;

    Ok(())
}

pub fn winit_dispatch<BackendData>(
    backend: &mut WinitGraphicsBackend<GlesRenderer>,
    winit: &mut WinitEventLoop,
    data: &mut NoWayData<BackendData>,
    output: &Output,
    damage_tracked_renderer: &mut OutputDamageTracker,
    full_redraw: &mut u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let display = &mut data.display;
    let state = &mut data.state;

    let res = winit.dispatch_new_events(|event| match event {
        WinitEvent::Resized { size, .. } => {
            output.change_current_state(
                Some(Mode {
                    size,
                    refresh: 60_000,
                }),
                None,
                None,
                None,
            );
            tracing::debug!("Resized to {:?}", size);
        }
        _ => (),
    });

    if let Err(WinitError::WindowClosed) = res {
        return Ok(());
    } else {
        res?;
    }

    *full_redraw = full_redraw.saturating_sub(1);

    let size = backend.window_size().physical_size;
    let damage = Rectangle::from_loc_and_size((0, 0), size);

    backend.bind()?;
    smithay::desktop::space::render_output::<_, WaylandSurfaceRenderElement<GlesRenderer>, _, _>(
        output,
        backend.renderer(),
        0,
        [&state.compositor.space],
        &[],
        damage_tracked_renderer,
        [0.1, 0.1, 0.1, 1.0],
    )?;
    backend.submit(Some(&[damage]))?;

    state.compositor.space.elements().for_each(|window| {
        window.send_frame(
            output,
            state.start_time.elapsed(),
            Some(Duration::ZERO),
            |_, _| Some(output.clone()),
        )
    });

    state.compositor.space.refresh();
    display.flush_clients()?;

    Ok(())
}
