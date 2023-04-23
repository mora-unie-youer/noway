use smithay::delegate_input_method_manager;

use crate::{backend::Backend, state::NoWayState};

delegate_input_method_manager!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
