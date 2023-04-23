use smithay::delegate_virtual_keyboard_manager;

use crate::{backend::Backend, state::NoWayState};

delegate_virtual_keyboard_manager!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
