use smithay::{
    delegate_compositor,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::compositor::{CompositorHandler, CompositorState},
};

use crate::state::NoWayState;

impl<BackendData: 'static> CompositorHandler for NoWayState<BackendData> {
    fn compositor_state(&mut self) -> &mut CompositorState {
        todo!()
    }

    fn commit(&mut self, surface: &WlSurface) {
        todo!()
    }
}

delegate_compositor!(@<BackendData: 'static> NoWayState<BackendData>);
