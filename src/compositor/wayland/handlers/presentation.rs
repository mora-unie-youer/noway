use smithay::delegate_presentation;

use crate::state::NoWayState;

delegate_presentation!(@<BackendData: 'static> NoWayState<BackendData>);
