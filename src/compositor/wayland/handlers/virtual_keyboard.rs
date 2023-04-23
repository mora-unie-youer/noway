use smithay::delegate_virtual_keyboard_manager;

use crate::state::NoWayState;

delegate_virtual_keyboard_manager!(@<BackendData: 'static> NoWayState<BackendData>);
