use smithay::delegate_output;

use crate::{backend::Backend, state::NoWayState};

delegate_output!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
