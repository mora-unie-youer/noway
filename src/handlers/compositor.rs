use smithay::{
    delegate_compositor,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::compositor::{CompositorHandler, CompositorState},
};

use crate::state::NoWayState;

impl CompositorHandler for NoWayState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn commit(&mut self, _surface: &WlSurface) {
        todo!()
    }
}

delegate_compositor!(NoWayState);
