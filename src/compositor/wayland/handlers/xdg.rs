use smithay::{
    delegate_xdg_activation, delegate_xdg_decoration, delegate_xdg_shell,
    reexports::{
        wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode,
        wayland_server::protocol::{wl_seat::WlSeat, wl_surface::WlSurface},
    },
    utils::Serial,
    wayland::{
        shell::xdg::{
            decoration::XdgDecorationHandler, PopupSurface, PositionerState, ToplevelSurface,
            XdgShellHandler, XdgShellState,
        },
        xdg_activation::{
            XdgActivationHandler, XdgActivationState, XdgActivationToken, XdgActivationTokenData,
        },
    },
};

use crate::state::NoWayState;

impl<BackendData: 'static> XdgActivationHandler for NoWayState<BackendData> {
    fn activation_state(&mut self) -> &mut XdgActivationState {
        todo!()
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

delegate_xdg_activation!(@<BackendData: 'static> NoWayState<BackendData>);

impl<BackendData: 'static> XdgDecorationHandler for NoWayState<BackendData> {
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        todo!()
    }

    fn request_mode(&mut self, toplevel: ToplevelSurface, mode: Mode) {
        todo!()
    }

    fn unset_mode(&mut self, toplevel: ToplevelSurface) {
        todo!()
    }
}

delegate_xdg_decoration!(@<BackendData: 'static> NoWayState<BackendData>);

impl<BackendData: 'static> XdgShellHandler for NoWayState<BackendData> {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        todo!()
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        todo!()
    }

    fn new_popup(&mut self, surface: PopupSurface, positioner: PositionerState) {
        todo!()
    }

    fn grab(&mut self, surface: PopupSurface, seat: WlSeat, serial: Serial) {
        todo!()
    }
}

delegate_xdg_shell!(@<BackendData: 'static> NoWayState<BackendData>);
