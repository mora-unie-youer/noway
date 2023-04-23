use std::cell::RefCell;

use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    delegate_compositor,
    desktop::{layer_map_for_output, Space, Window, WindowSurfaceType},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Rectangle},
    wayland::{
        compositor::{
            get_parent, is_sync_subsurface, with_states, with_surface_tree_upward,
            CompositorHandler, CompositorState, TraversalAction,
        },
        seat::WaylandFocus,
        shell::{wlr_layer::LayerSurfaceData, xdg::XdgToplevelSurfaceData},
    },
};

use crate::{backend::Backend, state::NoWayState};

#[derive(Default)]
pub struct SurfaceData {
    pub geometry: Option<Rectangle<i32, Logical>>,
}

fn ensure_initial_configure(surface: &WlSurface, space: &Space<Window>) {
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

    if let Some(window) = space
        .elements()
        .find(|window| window.wl_surface().map(|s| s == *surface).unwrap_or(false))
        .cloned()
    {
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

        return;
    }

    if let Some(output) = space.outputs().find(|o| {
        let map = layer_map_for_output(o);
        map.layer_for_surface(surface, WindowSurfaceType::TOPLEVEL)
            .is_some()
    }) {
        let initial_configure_sent = with_states(surface, |states| {
            states
                .data_map
                .get::<LayerSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .initial_configure_sent
        });

        let mut map = layer_map_for_output(output);
        map.arrange();

        if !initial_configure_sent {
            let layer = map
                .layer_for_surface(surface, WindowSurfaceType::TOPLEVEL)
                .unwrap();

            layer.layer_surface().send_configure();
        }
    };
}

impl<BackendData: Backend + 'static> NoWayState<BackendData> {
    pub fn window_for_surface(&self, surface: &WlSurface) -> Option<Window> {
        self.compositor
            .space
            .elements()
            .find(|window| window.wl_surface().map(|s| s == *surface).unwrap_or(false))
            .cloned()
    }
}

impl<BackendData: Backend + 'static> CompositorHandler for NoWayState<BackendData> {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor.compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler(surface);
        self.backend.early_import(surface);

        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }

            if let Some(window) = self.window_for_surface(&root) {
                window.on_commit();
            }
        }

        ensure_initial_configure(surface, &self.compositor.space);
    }
}

delegate_compositor!(@<BackendData: Backend + 'static> NoWayState<BackendData>);
