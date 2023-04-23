use smithay::{
    delegate_layer_shell,
    reexports::wayland_server::protocol::wl_output::WlOutput,
    wayland::shell::wlr_layer::{Layer, LayerSurface, WlrLayerShellHandler, WlrLayerShellState},
};

use crate::state::NoWayState;

impl<BackendData: 'static> WlrLayerShellHandler for NoWayState<BackendData> {
    fn shell_state(&mut self) -> &mut WlrLayerShellState {
        todo!()
    }

    fn new_layer_surface(
        &mut self,
        surface: LayerSurface,
        output: Option<WlOutput>,
        layer: Layer,
        namespace: String,
    ) {
        todo!()
    }
}

delegate_layer_shell!(@<BackendData: 'static> NoWayState<BackendData>);
