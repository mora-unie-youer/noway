use smithay::{
    delegate_xdg_shell,
    desktop::Window,
    input::{
        pointer::{Focus, GrabStartData},
        Seat,
    },
    reexports::wayland_server::{
        protocol::{wl_seat::WlSeat, wl_surface::WlSurface},
        Resource,
    },
    utils::Serial,
    wayland::{
        compositor::with_states,
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
            XdgToplevelSurfaceData,
        },
    },
};

use crate::{grabs::move_grab::MoveSurfaceGrab, state::NoWayState};

impl NoWayState {
    pub fn commit_xdg_surface(&self, surface: &WlSurface) {
        if let Some(window) = self.window_for_surface(surface) {
            let initial_configure_sent = with_states(surface, |states| {
                states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .initial_configure_sent
            });

            if !initial_configure_sent {
                window.toplevel().send_configure();
            }
        }
    }

    fn check_grab(
        &self,
        surface: &WlSurface,
        seat: &Seat<Self>,
        serial: Serial,
    ) -> Option<GrabStartData<NoWayState>> {
        let pointer = seat.get_pointer()?;
        if !pointer.has_grab(serial) {
            return None;
        }

        let start_data = pointer.grab_start_data()?;

        let (focus, _) = start_data.focus.as_ref()?;
        if !focus.id().same_client_as(&surface.id()) {
            return None;
        }

        Some(start_data)
    }
}

impl XdgShellHandler for NoWayState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window = Window::new(surface);
        self.space.map_element(window, (0, 0), false);
    }

    fn move_request(&mut self, surface: ToplevelSurface, seat: WlSeat, serial: Serial) {
        let seat = Seat::from_resource(&seat).unwrap();
        let surface = surface.wl_surface();

        if let Some(start_data) = self.check_grab(surface, &seat, serial) {
            let pointer = seat.get_pointer().unwrap();

            let window = self.window_for_surface(surface).unwrap();
            let initial_window_location = self.space.element_location(&window).unwrap();

            pointer.set_grab(
                self,
                MoveSurfaceGrab {
                    start_data,
                    window,
                    initial_window_location,
                },
                serial,
                Focus::Clear,
            );
        }
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        todo!()
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {
        todo!()
    }
}

delegate_xdg_shell!(NoWayState);
