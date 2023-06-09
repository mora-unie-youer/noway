use std::{
    ffi::OsString,
    os::fd::AsRawFd,
    sync::{Arc, Mutex},
    time::Instant,
};

use smithay::{
    desktop::Space,
    input::{pointer::CursorImageStatus, Seat, SeatState},
    reexports::{
        calloop::{generic::Generic, Interest, LoopHandle, LoopSignal, Mode, PostAction},
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            Display, DisplayHandle,
        },
    },
    utils::{Logical, Point},
    wayland::{
        compositor::CompositorState, data_device::DataDeviceState, output::OutputManagerState,
        shell::xdg::XdgShellState, shm::ShmState, socket::ListeningSocketSource,
    },
};

use crate::render::window::WindowElement;

pub struct ClientState;
impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}

#[derive(Debug)]
pub struct NoWayData {
    pub state: NoWayState,
    pub display: Display<NoWayState>,
}

#[derive(Debug)]
pub struct NoWayState {
    pub start_time: Instant,
    pub loop_handle: LoopHandle<'static, NoWayData>,
    pub loop_signal: LoopSignal,

    pub socket_name: OsString,
    pub space: Space<WindowElement>,

    pub cursor_status: Arc<Mutex<CursorImageStatus>>,
    pub pointer_location: Point<f64, Logical>,
    pub seat: Seat<Self>,

    pub display_handle: DisplayHandle,
    pub compositor_state: CompositorState,
    pub data_device_state: DataDeviceState,
    pub output_manager_state: OutputManagerState,
    pub seat_state: SeatState<Self>,
    pub shm_state: ShmState,
    pub xdg_shell_state: XdgShellState,
}

impl NoWayState {
    pub fn try_new(
        loop_handle: LoopHandle<'static, NoWayData>,
        loop_signal: LoopSignal,
        display: &mut Display<Self>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let start_time = Instant::now();

        let socket_name = Self::init_wayland_listener(&loop_handle, display)?;
        let space = Space::default();

        let dh = display.handle();
        let compositor_state = CompositorState::new::<Self>(&dh);
        let data_device_state = DataDeviceState::new::<Self>(&dh);
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);
        let mut seat_state = SeatState::new();
        let shm_state = ShmState::new::<Self>(&dh, vec![]);
        let xdg_shell_state = XdgShellState::new::<Self>(&dh);

        let mut seat = seat_state.new_wl_seat(&dh, "winit");
        seat.add_keyboard(Default::default(), 200, 50).unwrap();
        seat.add_pointer();

        Ok(Self {
            start_time,
            loop_handle,
            loop_signal,

            socket_name,
            space,

            cursor_status: Arc::new(Mutex::new(CursorImageStatus::Default)),
            pointer_location: (100.0, 100.0).into(),
            seat,

            display_handle: dh,
            compositor_state,
            data_device_state,
            output_manager_state,
            seat_state,
            shm_state,
            xdg_shell_state,
        })
    }

    fn init_wayland_listener(
        handle: &LoopHandle<'static, NoWayData>,
        display: &mut Display<Self>,
    ) -> Result<OsString, Box<dyn std::error::Error>> {
        let listening_socket = ListeningSocketSource::new_auto().unwrap();
        let socket_name = listening_socket.socket_name().to_os_string();

        handle.insert_source(listening_socket, move |client_stream, _, state| {
            state
                .display
                .handle()
                .insert_client(client_stream, Arc::new(ClientState))
                .unwrap();
        })?;

        handle.insert_source(
            Generic::new(
                display.backend().poll_fd().as_raw_fd(),
                Interest::READ,
                Mode::Level,
            ),
            |_, _, state| {
                state.display.dispatch_clients(&mut state.state).unwrap();
                Ok(PostAction::Continue)
            },
        )?;

        Ok(socket_name)
    }
}
