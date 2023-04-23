use smithay::{
    delegate_primary_selection,
    wayland::primary_selection::{PrimarySelectionHandler, PrimarySelectionState},
};

use crate::{backend::Backend, state::NoWayState};

impl<BackendData: Backend + 'static> PrimarySelectionHandler for NoWayState<BackendData> {
    fn primary_selection_state(&self) -> &PrimarySelectionState {
        todo!()
    }
}

delegate_primary_selection!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
