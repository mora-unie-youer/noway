use smithay::delegate_output;

use crate::state::NoWayState;

delegate_output!(@<BackendData: 'static> NoWayState<BackendData>);
