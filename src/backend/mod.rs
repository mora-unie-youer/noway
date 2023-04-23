use smithay::{output::Output, reexports::wayland_server::protocol::wl_surface::WlSurface};

pub mod winit;

pub trait Backend {
    fn seat_name(&self) -> String;
    fn early_import(&mut self, surface: &WlSurface);
    fn reset_buffers(&mut self, output: &Output);
}
