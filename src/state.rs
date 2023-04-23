use std::time::Instant;

use smithay::reexports::{
    calloop::{LoopHandle, LoopSignal},
    wayland_server::Display,
};

use crate::{backend::Backend, compositor::state::NoWayCompositorState};

#[derive(Debug)]
pub struct NoWayData<BackendData: Backend + 'static> {
    pub state: NoWayState<BackendData>,
    pub display: Display<NoWayState<BackendData>>,
}

#[derive(Debug)]
pub struct NoWayState<BackendData: Backend + 'static> {
    pub start_time: Instant,
    pub handle: LoopHandle<'static, NoWayData<BackendData>>,
    pub signal: LoopSignal,

    pub backend: BackendData,
    pub compositor: NoWayCompositorState<BackendData>,
}

impl<BackendData: Backend + 'static> NoWayState<BackendData> {
    pub fn new(
        handle: LoopHandle<'static, NoWayData<BackendData>>,
        signal: LoopSignal,
        display: &mut Display<Self>,
        backend: BackendData,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let compositor = NoWayCompositorState::<BackendData>::new(&handle, display)?;
        std::env::set_var("WAYLAND_DISPLAY", &compositor.socket_name);

        Ok(Self {
            start_time,
            handle,
            signal,

            backend,
            compositor,
        })
    }
}
