use std::cell::RefCell;

use smithay::{
    delegate_xdg_shell,
    desktop::{space::SpaceElement, Window},
    input::{
        pointer::{Focus, GrabStartData},
        Seat,
    },
    reexports::{
        wayland_protocols::xdg::shell::server::xdg_toplevel,
        wayland_server::{
            protocol::{
                wl_seat::{self, WlSeat},
                wl_surface::WlSurface,
            },
            Resource,
        },
    },
    utils::Serial,
    wayland::{
        compositor::{with_states, with_surface_tree_upward, TraversalAction},
        seat::WaylandFocus,
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
            XdgToplevelSurfaceData,
        },
    },
};

use crate::{
    grabs::{
        move_grab::MoveSurfaceGrab,
        resize_grab::{ResizeData, ResizeState, ResizeSurfaceGrab},
        SurfaceData,
    },
    render::window::WindowElement,
    state::NoWayState,
};

impl NoWayState {
    pub fn commit_xdg_surface(&self, surface: &WlSurface) {
        with_surface_tree_upward(
            surface,
            (),
            |_, _, _| TraversalAction::DoChildren(()),
            |_, states, _| {
                states
                    .data_map
                    .insert_if_missing(|| RefCell::new(SurfaceData::default()));
            },
            |_, _, _| true,
        );

        if let Some(WindowElement::Xdg(window)) = self.window_for_surface(surface) {
            let initial_configure_sent = with_states(surface, |states| {
                states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .initial_configure_sent
            });

            if !initial_configure_sent {
                window.toplevel().send_configure();
            }
        }

        with_states(surface, |states| {
            let mut data = states
                .data_map
                .get::<RefCell<SurfaceData>>()
                .unwrap()
                .borrow_mut();
            if let ResizeState::WaitingForCommit(_) = data.resize_state {
                data.resize_state = ResizeState::NotResizing;
            }
        });
    }

    fn check_grab(
        &self,
        surface: &WlSurface,
        seat: &Seat<Self>,
        serial: Serial,
    ) -> Option<GrabStartData<NoWayState>> {
        let pointer = seat.get_pointer()?;
        if !pointer.has_grab(serial) {
            return None;
        }

        let start_data = pointer.grab_start_data()?;

        let (focus, _) = start_data.focus.as_ref()?;
        if !focus.id().same_client_as(&surface.id()) {
            return None;
        }

        Some(start_data)
    }
}

impl XdgShellHandler for NoWayState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window = WindowElement::Xdg(Window::new(surface));
        self.space.map_element(window, (0, 0), false);
    }

    fn move_request(&mut self, surface: ToplevelSurface, seat: WlSeat, serial: Serial) {
        let seat = Seat::from_resource(&seat).unwrap();
        let surface = surface.wl_surface();

        if let Some(start_data) = self.check_grab(surface, &seat, serial) {
            let pointer = seat.get_pointer().unwrap();

            let window = self.window_for_surface(surface).unwrap();
            let initial_window_location = self.space.element_location(&window).unwrap();

            pointer.set_grab(
                self,
                MoveSurfaceGrab {
                    start_data,
                    window,
                    initial_window_location,
                },
                serial,
                Focus::Clear,
            );
        }
    }

    fn resize_request(
        &mut self,
        surface: ToplevelSurface,
        seat: wl_seat::WlSeat,
        serial: Serial,
        edges: xdg_toplevel::ResizeEdge,
    ) {
        let seat: Seat<Self> = Seat::from_resource(&seat).unwrap();
        // TODO: touch resize.
        let pointer = seat.get_pointer().unwrap();

        // Check that this surface has a click grab.
        if !pointer.has_grab(serial) {
            return;
        }

        let start_data = pointer.grab_start_data().unwrap();

        let window = self.window_for_surface(surface.wl_surface()).unwrap();

        // If the focus was for a different surface, ignore the request.
        if start_data.focus.is_none()
            || !start_data
                .focus
                .as_ref()
                .unwrap()
                .0
                .same_client_as(&surface.wl_surface().id())
        {
            return;
        }

        let geometry = window.geometry();
        let loc = self.space.element_location(&window).unwrap();
        let (initial_window_location, initial_window_size) = (loc, geometry.size);

        with_states(surface.wl_surface(), move |states| {
            states
                .data_map
                .get::<RefCell<SurfaceData>>()
                .unwrap()
                .borrow_mut()
                .resize_state = ResizeState::Resizing(ResizeData {
                edges: edges.into(),
                initial_window_location,
                initial_window_size,
            });
        });

        let grab = ResizeSurfaceGrab {
            start_data,
            window,
            edges: edges.into(),
            initial_window_location,
            initial_window_size,
            last_window_size: initial_window_size,
        };

        pointer.set_grab(self, grab, serial, Focus::Clear);
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        todo!()
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {
        todo!()
    }
}

delegate_xdg_shell!(NoWayState);
