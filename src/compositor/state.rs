use std::{ffi::OsString, os::fd::AsRawFd, sync::Arc};

use smithay::{
    desktop::{Space, Window},
    input::SeatState,
    reexports::{
        calloop::{generic::Generic, Interest, LoopHandle, Mode, PostAction},
        wayland_protocols_misc::server_decoration::server::org_kde_kwin_server_decoration_manager::Mode as KdeMode,
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            Display, DisplayHandle,
        },
    },
    utils::{Clock, Monotonic},
    wayland::{
        compositor::CompositorState,
        data_device::DataDeviceState,
        dmabuf::DmabufState,
        fractional_scale::FractionalScaleManagerState,
        input_method::InputMethodManagerState,
        keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState,
        output::OutputManagerState,
        presentation::PresentationState,
        primary_selection::PrimarySelectionState,
        shell::{
            kde::decoration::KdeDecorationState,
            wlr_layer::WlrLayerShellState,
            xdg::{decoration::XdgDecorationState, XdgShellState},
        },
        shm::ShmState,
        socket::ListeningSocketSource,
        text_input::TextInputManagerState,
        viewporter::ViewporterState,
        virtual_keyboard::VirtualKeyboardManagerState,
        xdg_activation::XdgActivationState,
    },
};

use crate::state::{NoWayData, NoWayState};

#[derive(Debug)]
pub struct NoWayCompositorState<BackendData: 'static> {
    pub socket_name: OsString,
    pub display_handle: DisplayHandle,

    pub clock: Clock<Monotonic>,
    pub space: Space<Window>,

    // Wayland states
    pub compositor_state: CompositorState,
    pub data_device_state: DataDeviceState,
    pub dmabuf_state: DmabufState,
    pub fractional_scale_manager_state: FractionalScaleManagerState,
    pub input_method_manager_state: InputMethodManagerState,
    pub kde_decoration_state: KdeDecorationState,
    pub keyboard_shortcuts_inhibit_state: KeyboardShortcutsInhibitState,
    pub layer_shell_state: WlrLayerShellState,
    pub output_manager_state: OutputManagerState,
    pub presentation_state: PresentationState,
    pub primary_selection_state: PrimarySelectionState,
    pub seat_state: SeatState<NoWayState<BackendData>>,
    pub shm_state: ShmState,
    pub text_input_manager_state: TextInputManagerState,
    pub viewporter_state: ViewporterState,
    pub virtual_keyboard_manager_state: VirtualKeyboardManagerState,
    pub xdg_activation_state: XdgActivationState,
    pub xdg_decoration_state: XdgDecorationState,
    pub xdg_shell_state: XdgShellState,
}

impl<BackendData: 'static> NoWayCompositorState<BackendData> {
    pub fn new(
        handle: &LoopHandle<'static, NoWayData<BackendData>>,
        display: &mut Display<NoWayState<BackendData>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let socket_name = Self::init_wayland_socket(handle, display)?;
        let clock = Clock::new()?;

        let dh = display.handle();
        let compositor_state = CompositorState::new::<NoWayState<BackendData>>(&dh);
        let data_device_state = DataDeviceState::new::<NoWayState<BackendData>>(&dh);
        let dmabuf_state = DmabufState::new();
        let fractional_scale_manager_state =
            FractionalScaleManagerState::new::<NoWayState<BackendData>>(&dh);
        let input_method_manager_state =
            InputMethodManagerState::new::<NoWayState<BackendData>>(&dh);
        let kde_decoration_state =
            KdeDecorationState::new::<NoWayState<BackendData>>(&dh, KdeMode::Client);
        let keyboard_shortcuts_inhibit_state =
            KeyboardShortcutsInhibitState::new::<NoWayState<BackendData>>(&dh);
        let layer_shell_state = WlrLayerShellState::new::<NoWayState<BackendData>>(&dh);
        let output_manager_state =
            OutputManagerState::new_with_xdg_output::<NoWayState<BackendData>>(&dh);
        let presentation_state =
            PresentationState::new::<NoWayState<BackendData>>(&dh, clock.id() as u32);
        let primary_selection_state = PrimarySelectionState::new::<NoWayState<BackendData>>(&dh);
        let seat_state = SeatState::<NoWayState<BackendData>>::new();
        let shm_state = ShmState::new::<NoWayState<BackendData>>(&dh, vec![]);
        let text_input_manager_state = TextInputManagerState::new::<NoWayState<BackendData>>(&dh);
        let viewporter_state = ViewporterState::new::<NoWayState<BackendData>>(&dh);
        let virtual_keyboard_manager_state =
            VirtualKeyboardManagerState::new::<NoWayState<BackendData>, _>(&dh, |_| true);
        let xdg_activation_state = XdgActivationState::new::<NoWayState<BackendData>>(&dh);
        let xdg_decoration_state = XdgDecorationState::new::<NoWayState<BackendData>>(&dh);
        let xdg_shell_state = XdgShellState::new::<NoWayState<BackendData>>(&dh);

        Ok(Self {
            socket_name,
            display_handle: dh,

            clock,
            space: Space::default(),

            compositor_state,
            data_device_state,
            dmabuf_state,
            fractional_scale_manager_state,
            input_method_manager_state,
            kde_decoration_state,
            keyboard_shortcuts_inhibit_state,
            layer_shell_state,
            output_manager_state,
            presentation_state,
            primary_selection_state,
            seat_state,
            shm_state,
            text_input_manager_state,
            viewporter_state,
            virtual_keyboard_manager_state,
            xdg_activation_state,
            xdg_decoration_state,
            xdg_shell_state,
        })
    }

    fn init_wayland_socket(
        handle: &LoopHandle<'static, NoWayData<BackendData>>,
        display: &mut Display<NoWayState<BackendData>>,
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

struct ClientState;
impl ClientData for ClientState {
    /// Notification that a client was initialized
    fn initialized(&self, _client_id: ClientId) {}
    /// Notification that a client is disconnected
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}
