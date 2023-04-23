use smithay::{
    delegate_fractional_scale, reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::fractional_scale::FractionalScaleHandler,
};

use crate::{backend::Backend, state::NoWayState};

impl<BackendData: Backend + 'static> FractionalScaleHandler for NoWayState<BackendData> {
    fn new_fractional_scale(&mut self, surface: WlSurface) {
        todo!()
    }
}

delegate_fractional_scale!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
