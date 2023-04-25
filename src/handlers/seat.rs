use smithay::{
    delegate_seat,
    input::{SeatHandler, SeatState},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
};

use crate::state::NoWayState;

impl SeatHandler for NoWayState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

delegate_seat!(NoWayState);
