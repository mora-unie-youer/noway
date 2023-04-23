use std::time::Duration;

use smithay::{
    backend::{
        allocator::dmabuf::Dmabuf,
        egl::EGLDevice,
        renderer::{
            damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement,
            gles::GlesRenderer, ImportDma,
        },
        winit::{self, WinitError, WinitEvent, WinitEventLoop, WinitGraphicsBackend},
    },
    delegate_dmabuf,
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{
        calloop::{
            timer::{TimeoutAction, Timer},
            EventLoop,
        },
        wayland_server::{protocol::wl_surface::WlSurface, Display},
    },
    utils::{Rectangle, Transform},
    wayland::dmabuf::{
        DmabufFeedback, DmabufFeedbackBuilder, DmabufGlobal, DmabufHandler, DmabufState,
        ImportError,
    },
};

use crate::{
    backend::Backend,
    state::{NoWayData, NoWayState},
};

#[derive(Debug)]
pub struct WinitBackendData {
    backend: WinitGraphicsBackend<GlesRenderer>,
    damage_tracker: OutputDamageTracker,
    dmabuf_state: (DmabufState, DmabufGlobal, Option<DmabufFeedback>),
    full_redraw: u8,
}

impl DmabufHandler for NoWayState<WinitBackendData> {
    fn dmabuf_state(&mut self) -> &mut DmabufState {
        &mut self.backend.dmabuf_state.0
    }

    fn dmabuf_imported(
        &mut self,
        _global: &DmabufGlobal,
        dmabuf: Dmabuf,
    ) -> Result<(), ImportError> {
        self.backend
            .backend
            .renderer()
            .import_dmabuf(&dmabuf, None)
            .map(|_| ())
            .map_err(|_| ImportError::Failed)
    }
}

delegate_dmabuf!(NoWayState<WinitBackendData>);

impl Backend for WinitBackendData {
    fn seat_name(&self) -> String {
        "winit".into()
    }

    fn early_import(&mut self, _surface: &WlSurface) {}

    fn reset_buffers(&mut self, _output: &Output) {
        self.full_redraw = 4;
    }
}

pub fn init_winit() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop = EventLoop::try_new()?;
    let mut display = Display::new()?;

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

    let render_node = EGLDevice::device_for_display(backend.renderer().egl_context().display())
        .and_then(|device| device.try_get_render_node());

    let dmabuf_default_feedback = match render_node {
        Ok(Some(node)) => {
            let dmabuf_formats = backend.renderer().dmabuf_formats().collect::<Vec<_>>();
            let dmabuf_default_feedback = DmabufFeedbackBuilder::new(node.dev_id(), dmabuf_formats)
                .build()
                .unwrap();
            Some(dmabuf_default_feedback)
        }
        Ok(None) => {
            tracing::warn!("Failed to query render node, dmabuf will use v3");
            None
        }
        Err(err) => {
            tracing::warn!(?err, "Failed to egl device for display, dmabuf will use v3");
            None
        }
    };

    let dmabuf_state = if let Some(default_feedback) = dmabuf_default_feedback {
        let mut dmabuf_state = DmabufState::new();
        let dmabuf_global = dmabuf_state
            .create_global_with_default_feedback::<NoWayState<WinitBackendData>>(
                &display.handle(),
                &default_feedback,
            );
        (dmabuf_state, dmabuf_global, Some(default_feedback))
    } else {
        let dmabuf_formats = backend.renderer().dmabuf_formats().collect::<Vec<_>>();
        let mut dmabuf_state = DmabufState::new();
        let dmabuf_global = dmabuf_state
            .create_global::<NoWayState<WinitBackendData>>(&display.handle(), dmabuf_formats);
        (dmabuf_state, dmabuf_global, None)
    };

    let data = WinitBackendData {
        backend,
        dmabuf_state,
        damage_tracker: OutputDamageTracker::from_output(&output),
        full_redraw: 0,
    };

    let state = NoWayState::new(
        event_loop.handle(),
        event_loop.get_signal(),
        &mut display,
        data,
    )?;

    let timer = Timer::immediate();
    event_loop
        .handle()
        .insert_source(timer, move |_, _, data| {
            winit_dispatch(&mut winit, data, &output).unwrap();
            TimeoutAction::ToDuration(Duration::from_millis(16))
        })?;

    let mut data = NoWayData { state, display };
    event_loop.run(None, &mut data, move |_| {})?;

    Ok(())
}

pub fn winit_dispatch(
    winit: &mut WinitEventLoop,
    data: &mut NoWayData<WinitBackendData>,
    output: &Output,
) -> Result<(), Box<dyn std::error::Error>> {
    let display = &mut data.display;
    let state = &mut data.state;
    let backend = &mut state.backend;

    let res = winit.dispatch_new_events(|event| {
        if let WinitEvent::Resized { size, .. } = event {
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
    });

    if let Err(WinitError::WindowClosed) = res {
        return Ok(());
    } else {
        res?;
    }

    backend.full_redraw = backend.full_redraw.saturating_sub(1);

    let size = backend.backend.window_size().physical_size;
    let damage = Rectangle::from_loc_and_size((0, 0), size);

    backend.backend.bind()?;
    smithay::desktop::space::render_output::<_, WaylandSurfaceRenderElement<GlesRenderer>, _, _>(
        output,
        backend.backend.renderer(),
        0,
        [&state.compositor.space],
        &[],
        &mut backend.damage_tracker,
        [0.1, 0.1, 0.1, 1.0],
    )?;
    backend.backend.submit(Some(&[damage]))?;

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
