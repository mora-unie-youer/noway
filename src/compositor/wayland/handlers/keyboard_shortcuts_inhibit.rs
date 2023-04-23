use smithay::{
    delegate_keyboard_shortcuts_inhibit,
    wayland::keyboard_shortcuts_inhibit::{
        KeyboardShortcutsInhibitHandler, KeyboardShortcutsInhibitState,
    },
};

use crate::state::NoWayState;

impl<BackendData: 'static> KeyboardShortcutsInhibitHandler for NoWayState<BackendData> {
    fn keyboard_shortcuts_inhibit_state(&mut self) -> &mut KeyboardShortcutsInhibitState {
        todo!()
    }
}

delegate_keyboard_shortcuts_inhibit!(@<BackendData: 'static> NoWayState<BackendData>);
