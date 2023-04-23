use smithay::delegate_text_input_manager;

use crate::{backend::Backend, state::NoWayState};

delegate_text_input_manager!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
