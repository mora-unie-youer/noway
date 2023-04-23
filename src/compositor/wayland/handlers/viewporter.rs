use smithay::delegate_viewporter;

use crate::{backend::Backend, state::NoWayState};

delegate_viewporter!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
