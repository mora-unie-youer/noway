use smithay::delegate_viewporter;

use crate::state::NoWayState;

delegate_viewporter!(@<BackendData: 'static> NoWayState<BackendData>);
