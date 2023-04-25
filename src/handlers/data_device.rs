use smithay::{
    delegate_data_device,
    wayland::data_device::{
        ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
    },
};

use crate::state::NoWayState;

impl ClientDndGrabHandler for NoWayState {}
impl ServerDndGrabHandler for NoWayState {}
impl DataDeviceHandler for NoWayState {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

delegate_data_device!(NoWayState);
