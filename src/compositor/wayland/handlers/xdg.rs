use smithay::{
    delegate_xdg_activation, delegate_xdg_decoration, delegate_xdg_shell,
    desktop::Window,
    reexports::{
        wayland_protocols::xdg::{
            decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode, shell::server::xdg_toplevel,
        },
        wayland_server::protocol::{wl_seat::WlSeat, wl_surface::WlSurface},
    },
    utils::Serial,
    wayland::{
        compositor::with_states,
        shell::xdg::{
            decoration::XdgDecorationHandler, PopupSurface, PositionerState, ToplevelSurface,
            XdgShellHandler, XdgShellState, XdgToplevelSurfaceData,
        },
        xdg_activation::{
            XdgActivationHandler, XdgActivationState, XdgActivationToken, XdgActivationTokenData,
        },
    },
};

use crate::{backend::Backend, state::NoWayState};

impl<BackendData: Backend + 'static> XdgActivationHandler for NoWayState<BackendData> {
    fn activation_state(&mut self) -> &mut XdgActivationState {
        &mut self.compositor.xdg_activation_state
    }

    fn request_activation(
        &mut self,
        token: XdgActivationToken,
        token_data: XdgActivationTokenData,
        surface: WlSurface,
    ) {
        todo!()
    }

    fn destroy_activation(
        &mut self,
        token: XdgActivationToken,
        token_data: XdgActivationTokenData,
        surface: WlSurface,
    ) {
        todo!()
    }
}

delegate_xdg_activation!(@<BackendData: Backend + 'static> NoWayState<BackendData>);

impl<BackendData: Backend + 'static> XdgDecorationHandler for NoWayState<BackendData> {
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ClientSide);
        });
    }

    fn request_mode(&mut self, toplevel: ToplevelSurface, mode: Mode) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(mode);
            // state.decoration_mode = Some(match mode {
            //     DecorationMode::ServerSide => Mode::ServerSide,
            //     _ => Mode::ClientSide,
            // });
        });

        let initial_configure_sent = with_states(toplevel.wl_surface(), |states| {
            states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .initial_configure_sent
        });

        if initial_configure_sent {
            toplevel.send_configure();
        }
    }

    fn unset_mode(&mut self, toplevel: ToplevelSurface) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ClientSide);
        });

        let initial_configure_sent = with_states(toplevel.wl_surface(), |states| {
            states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .initial_configure_sent
        });

        if initial_configure_sent {
            toplevel.send_configure();
        }
    }
}

delegate_xdg_decoration!(@<BackendData: Backend + 'static> NoWayState<BackendData>);

impl<BackendData: Backend + 'static> XdgShellHandler for NoWayState<BackendData> {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.compositor.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window = Window::new(surface);
        window.toplevel().with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::TiledLeft);
            state.states.set(xdg_toplevel::State::TiledBottom);
            state.states.set(xdg_toplevel::State::TiledRight);
            state.states.set(xdg_toplevel::State::TiledTop);
        });
        window.toplevel().send_configure();
        self.compositor.space.map_element(window, (0, 0), false);
    }

    fn new_popup(&mut self, surface: PopupSurface, positioner: PositionerState) {
        todo!()
    }

    fn grab(&mut self, surface: PopupSurface, seat: WlSeat, serial: Serial) {
        todo!()
    }
}

delegate_xdg_shell!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
