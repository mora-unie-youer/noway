use smithay::{
    delegate_xdg_shell,
    desktop::Window,
    reexports::wayland_server::protocol::{wl_seat::WlSeat, wl_surface::WlSurface},
    utils::Serial,
    wayland::{
        compositor::with_states,
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
            XdgToplevelSurfaceData,
        },
    },
};

use crate::state::NoWayState;

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
}

impl XdgShellHandler for NoWayState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window = Window::new(surface);
        self.space.map_element(window, (0, 0), false);
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        todo!()
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {
        todo!()
    }
}

delegate_xdg_shell!(NoWayState);
