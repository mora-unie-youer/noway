use std::{sync::Mutex, time::Duration};

use smithay::{
    backend::{
        renderer::{
            damage::OutputDamageTracker,
            element::AsRenderElements,
            gles::{GlesRenderer, GlesTexture},
        },
        winit::{self, WinitError, WinitEvent, WinitEventLoop, WinitGraphicsBackend},
    },
    input::pointer::{CursorImageAttributes, CursorImageStatus},
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::calloop::{
        timer::{TimeoutAction, Timer},
        EventLoop,
    },
    utils::{IsAlive, Scale, Transform},
    wayland::compositor,
};

use crate::{
    render::{pointer::PointerElement, render_output},
    state::{NoWayData, NoWayState},
};

pub fn initialize_winit(
    event_loop: &mut EventLoop<NoWayData>,
    data: &mut NoWayData,
) -> Result<(), Box<dyn std::error::Error>> {
    let display = &mut data.display;
    let state = &mut data.state;

    let (mut backend, mut winit) = winit::init()?;

    let mode = Mode {
        size: backend.window_size().physical_size,
        refresh: 60_000,
    };

    let output = Output::new(
        "winit".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Smithay".into(),
            model: "Winit".into(),
        },
    );
    let _global = output.create_global::<NoWayState>(&display.handle());
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);
    state.space.map_output(&output, (0, 0));

    let mut damage_tracker = OutputDamageTracker::from_output(&output);
    let mut pointer_element = PointerElement::default();
    let mut full_redraw = 4u8;

    std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);

    let timer = Timer::immediate();
    event_loop
        .handle()
        .insert_source(timer, move |_, _, data| {
            winit_dispatch(
                &mut backend,
                &mut winit,
                data,
                &output,
                &mut damage_tracker,
                &mut pointer_element,
                &mut full_redraw,
            )
            .unwrap();
            TimeoutAction::ToDuration(Duration::from_millis(16))
        })?;

    Ok(())
}

pub fn winit_dispatch(
    backend: &mut WinitGraphicsBackend<GlesRenderer>,
    winit: &mut WinitEventLoop,
    data: &mut NoWayData,
    output: &Output,
    damage_tracker: &mut OutputDamageTracker,
    pointer_element: &mut PointerElement<GlesTexture>,
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
        }
        WinitEvent::Input(event) => state.process_input_event(event),
        _ => (),
    });

    if let Err(WinitError::WindowClosed) = res {
        state.loop_signal.stop();
        return Ok(());
    } else {
        res?;
    }

    *full_redraw = full_redraw.saturating_sub(1);
    let scale = Scale::from(output.current_scale().fractional_scale());

    let mut cursor_guard = state.cursor_status.lock().unwrap();
    if let CursorImageStatus::Surface(surface) = cursor_guard.clone() {
        if !surface.alive() {
            *cursor_guard = CursorImageStatus::Default;
        }
    }
    pointer_element.set_status(cursor_guard.clone());
    let cursor_visible = !matches!(*cursor_guard, CursorImageStatus::Surface(_));

    let cursor_hotspot = if let CursorImageStatus::Surface(ref surface) = *cursor_guard {
        compositor::with_states(surface, |states| {
            states
                .data_map
                .get::<Mutex<CursorImageAttributes>>()
                .unwrap()
                .lock()
                .unwrap()
                .hotspot
        })
    } else {
        (0, 0).into()
    };
    let cursor_pos = state.pointer_location - cursor_hotspot.to_f64();
    let cursor_pos_scaled = cursor_pos.to_physical(scale).to_i32_round();

    backend.bind()?;
    let age = if *full_redraw > 0 {
        0
    } else {
        backend.buffer_age().unwrap_or(0)
    };

    let renderer = backend.renderer();
    let mut custom_elements = Vec::new();
    custom_elements.extend(pointer_element.render_elements(renderer, cursor_pos_scaled, scale));

    let (damage, _) = render_output(
        output,
        &state.space,
        custom_elements,
        renderer,
        damage_tracker,
        age,
    )?;

    if let Some(damage) = damage {
        backend.submit(Some(&damage))?;
    }

    state.space.elements().for_each(|window| {
        window.send_frame(
            output,
            state.start_time.elapsed(),
            Some(Duration::ZERO),
            |_, _| Some(output.clone()),
        )
    });

    backend.window().set_cursor_visible(cursor_visible);
    state.space.refresh();
    display.flush_clients()?;

    Ok(())
}
