use smithay::{
    delegate_kde_decoration,
    wayland::shell::kde::decoration::{KdeDecorationHandler, KdeDecorationState},
};

use crate::{backend::Backend, state::NoWayState};

impl<BackendData: Backend + 'static> KdeDecorationHandler for NoWayState<BackendData> {
    fn kde_decoration_state(&self) -> &KdeDecorationState {
        todo!()
    }
}

delegate_kde_decoration!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
