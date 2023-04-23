use smithay::{
    delegate_fractional_scale, reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::fractional_scale::FractionalScaleHandler,
};

use crate::state::NoWayState;

impl<BackendData: 'static> FractionalScaleHandler for NoWayState<BackendData> {
    fn new_fractional_scale(&mut self, surface: WlSurface) {
        todo!()
    }
}

delegate_fractional_scale!(@<BackendData: 'static> NoWayState<BackendData>);
