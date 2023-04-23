use smithay::{
    delegate_shm,
    reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    wayland::{
        buffer::BufferHandler,
        shm::{ShmHandler, ShmState},
    },
};

use crate::state::NoWayState;

impl<BackendData: 'static> BufferHandler for NoWayState<BackendData> {
    fn buffer_destroyed(&mut self, buffer: &WlBuffer) {
        todo!()
    }
}

impl<BackendData: 'static> ShmHandler for NoWayState<BackendData> {
    fn shm_state(&self) -> &ShmState {
        todo!()
    }
}

delegate_shm!(@<BackendData: 'static> NoWayState<BackendData>);
