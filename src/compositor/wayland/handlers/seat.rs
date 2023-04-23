use smithay::{
    delegate_seat,
    desktop::Window,
    input::{SeatHandler, SeatState},
};

use crate::{backend::Backend, state::NoWayState};

impl<BackendData: Backend + 'static> SeatHandler for NoWayState<BackendData> {
    // TODO: make complex focus target
    type KeyboardFocus = Window;
    type PointerFocus = Window;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.compositor.seat_state
    }
}

delegate_seat!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
