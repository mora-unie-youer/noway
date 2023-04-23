use smithay::{
    delegate_primary_selection,
    wayland::primary_selection::{PrimarySelectionHandler, PrimarySelectionState},
};

use crate::state::NoWayState;

impl<BackendData: 'static> PrimarySelectionHandler for NoWayState<BackendData> {
    fn primary_selection_state(&self) -> &PrimarySelectionState {
        todo!()
    }
}

delegate_primary_selection!(@<BackendData: 'static> NoWayState<BackendData>);
