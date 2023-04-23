use smithay::delegate_text_input_manager;

use crate::state::NoWayState;

delegate_text_input_manager!(@<BackendData: 'static> NoWayState<BackendData>);
