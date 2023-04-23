use smithay::{
    delegate_data_device,
    wayland::data_device::{
        ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
    },
};

use crate::{backend::Backend, state::NoWayState};

impl<BackendData: Backend + 'static> ClientDndGrabHandler for NoWayState<BackendData> {}
impl<BackendData: Backend + 'static> ServerDndGrabHandler for NoWayState<BackendData> {}
impl<BackendData: Backend + 'static> DataDeviceHandler for NoWayState<BackendData> {
    fn data_device_state(&self) -> &DataDeviceState {
        todo!()
    }
}

delegate_data_device!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
