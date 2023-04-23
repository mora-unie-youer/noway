use smithay::delegate_input_method_manager;

use crate::state::NoWayState;

delegate_input_method_manager!(@<BackendData: 'static> NoWayState<BackendData>);
