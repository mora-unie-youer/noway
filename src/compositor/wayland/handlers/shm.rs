use smithay::{
    delegate_shm,
    reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    wayland::{
        buffer::BufferHandler,
        shm::{ShmHandler, ShmState},
    },
};

use crate::{backend::Backend, state::NoWayState};

impl<BackendData: Backend + 'static> BufferHandler for NoWayState<BackendData> {
    fn buffer_destroyed(&mut self, buffer: &WlBuffer) {
        todo!()
    }
}

impl<BackendData: Backend + 'static> ShmHandler for NoWayState<BackendData> {
    fn shm_state(&self) -> &ShmState {
        todo!()
    }
}

delegate_shm!(@<BackendData: Backend + 'static> NoWayState<BackendData>);