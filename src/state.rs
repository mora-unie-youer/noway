use std::{ffi::OsString, os::fd::AsRawFd, sync::Arc, time::Instant};

use smithay::{
    desktop::{Space, Window},
    reexports::{
        calloop::{
            generic::Generic, Interest, LoopHandle, LoopSignal, Mode, PostAction,
        },
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            Display, DisplayHandle,
        },
    },
    wayland::{socket::ListeningSocketSource, output::OutputManagerState},
};

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
    pub space: Space<Window>,

    pub display_handle: DisplayHandle,
    pub output_manager_state: OutputManagerState,
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
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);

        Ok(Self {
            start_time,
            loop_handle,
            loop_signal,

            socket_name,
            space,
            
            display_handle: dh,
            output_manager_state,
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
