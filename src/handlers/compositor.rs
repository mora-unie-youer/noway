use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    delegate_compositor,
    desktop::{Window, WindowSurfaceType},
    input::pointer::PointerHandle,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Point},
    wayland::compositor::{get_parent, is_sync_subsurface, CompositorHandler, CompositorState},
};

use crate::state::NoWayState;

impl NoWayState {
    pub fn window_for_surface(&self, surface: &WlSurface) -> Option<Window> {
        self.space
            .elements()
            .find(|window| window.toplevel().wl_surface() == surface)
            .cloned()
    }

    pub fn window_under_pointer(
        &self,
        pointer: &PointerHandle<Self>,
    ) -> Option<(&Window, Point<i32, Logical>)> {
        let pos = pointer.current_location();
        self.space.element_under(pos)
    }

    pub fn surface_under_pointer(
        &self,
        pointer: &PointerHandle<Self>,
    ) -> Option<(WlSurface, Point<i32, Logical>)> {
        let pos = pointer.current_location();
        self.window_under_pointer(pointer)
            .and_then(|(window, location)| {
                window
                    .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                    .map(|(s, p)| (s, p + location))
            })
    }
}

impl CompositorHandler for NoWayState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler(surface);
        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }

            if let Some(window) = self.window_for_surface(surface) {
                window.on_commit();
            }
        };

        self.commit_xdg_surface(surface);
    }
}

delegate_compositor!(NoWayState);
