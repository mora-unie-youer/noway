use smithay::delegate_presentation;

use crate::{backend::Backend, state::NoWayState};

delegate_presentation!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
