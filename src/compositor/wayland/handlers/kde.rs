use smithay::{
    delegate_kde_decoration,
    wayland::shell::kde::decoration::{KdeDecorationHandler, KdeDecorationState},
};

use crate::state::NoWayState;

impl<BackendData: 'static> KdeDecorationHandler for NoWayState<BackendData> {
    fn kde_decoration_state(&self) -> &KdeDecorationState {
        todo!()
    }
}

delegate_kde_decoration!(@<BackendData: 'static> NoWayState<BackendData>);
