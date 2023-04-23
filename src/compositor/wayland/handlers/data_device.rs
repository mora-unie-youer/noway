use smithay::{
    delegate_data_device,
    wayland::data_device::{
        ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
    },
};

use crate::state::NoWayState;

impl<BackendData: 'static> ClientDndGrabHandler for NoWayState<BackendData> {}
impl<BackendData: 'static> ServerDndGrabHandler for NoWayState<BackendData> {}
impl<BackendData: 'static> DataDeviceHandler for NoWayState<BackendData> {
    fn data_device_state(&self) -> &DataDeviceState {
        todo!()
    }
}

delegate_data_device!(@<BackendData: 'static> NoWayState<BackendData>);
