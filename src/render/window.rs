use std::time::Duration;

use smithay::{
    backend::{
        input::KeyState,
        renderer::{
            element::{
                solid::SolidColorRenderElement, surface::WaylandSurfaceRenderElement,
                AsRenderElements,
            },
            ImportAll, ImportMem, Renderer, Texture,
        },
    },
    desktop::{
        space::SpaceElement,
        utils::{
            send_dmabuf_feedback_surface_tree, send_frames_surface_tree,
            take_presentation_feedback_surface_tree, under_from_surface_tree,
            with_surfaces_surface_tree, OutputPresentationFeedback,
        },
        Window, WindowSurfaceType,
    },
    input::{
        keyboard::{KeyboardTarget, KeysymHandle, ModifiersState},
        pointer::{AxisFrame, ButtonEvent, MotionEvent, PointerTarget, RelativeMotionEvent},
        Seat,
    },
    output::Output,
    reexports::{
        wayland_protocols::wp::presentation_time::server::wp_presentation_feedback,
        wayland_server::protocol::wl_surface::WlSurface,
    },
    render_elements,
    utils::{user_data::UserDataMap, IsAlive, Logical, Physical, Point, Rectangle, Scale, Serial},
    wayland::{
        compositor::SurfaceData as WlSurfaceData, dmabuf::DmabufFeedback, seat::WaylandFocus,
    },
    xwayland::X11Surface,
};

use crate::state::NoWayState;

#[derive(Debug, Clone, PartialEq)]
pub enum WindowElement {
    Xdg(Window),
    X11(X11Surface),
}

impl WindowElement {
    pub fn surface_under(
        &self,
        location: Point<f64, Logical>,
        window_type: WindowSurfaceType,
    ) -> Option<(WlSurface, Point<i32, Logical>)> {
        match self {
            Self::Xdg(w) => w.surface_under(location, window_type),
            Self::X11(w) => w
                .wl_surface()
                .and_then(|s| under_from_surface_tree(&s, location, (0, 0), window_type)),
        }
    }

    pub fn with_surfaces<F>(&self, processor: F)
    where
        F: FnMut(&WlSurface, &WlSurfaceData) + Copy,
    {
        match self {
            Self::Xdg(w) => w.with_surfaces(processor),
            Self::X11(w) => {
                if let Some(surface) = w.wl_surface() {
                    with_surfaces_surface_tree(&surface, processor);
                }
            }
        }
    }

    pub fn send_frame<T, F>(
        &self,
        output: &Output,
        time: T,
        throttle: Option<Duration>,
        primary_scan_out_output: F,
    ) where
        T: Into<Duration>,
        F: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
    {
        match self {
            Self::Xdg(w) => w.send_frame(output, time, throttle, primary_scan_out_output),
            Self::X11(w) => {
                if let Some(surface) = w.wl_surface() {
                    send_frames_surface_tree(
                        &surface,
                        output,
                        time,
                        throttle,
                        primary_scan_out_output,
                    );
                }
            }
        }
    }

    pub fn send_dmabuf_feedback<'a, P, F>(
        &self,
        output: &Output,
        primary_scan_out_output: P,
        select_dmabuf_feedback: F,
    ) where
        P: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
        F: Fn(&WlSurface, &WlSurfaceData) -> &'a DmabufFeedback + Copy,
    {
        match self {
            Self::Xdg(w) => {
                w.send_dmabuf_feedback(output, primary_scan_out_output, select_dmabuf_feedback)
            }
            Self::X11(w) => {
                if let Some(surface) = w.wl_surface() {
                    send_dmabuf_feedback_surface_tree(
                        &surface,
                        output,
                        primary_scan_out_output,
                        select_dmabuf_feedback,
                    )
                }
            }
        }
    }

    pub fn take_presentation_feedback<F1, F2>(
        &self,
        output_feedback: &mut OutputPresentationFeedback,
        primary_scan_out_output: F1,
        presentation_feedback_flags: F2,
    ) where
        F1: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
        F2: FnMut(&WlSurface, &WlSurfaceData) -> wp_presentation_feedback::Kind + Copy,
    {
        match self {
            Self::Xdg(w) => w.take_presentation_feedback(
                output_feedback,
                primary_scan_out_output,
                presentation_feedback_flags,
            ),
            Self::X11(w) => {
                if let Some(surface) = w.wl_surface() {
                    take_presentation_feedback_surface_tree(
                        &surface,
                        output_feedback,
                        primary_scan_out_output,
                        presentation_feedback_flags,
                    );
                }
            }
        }
    }

    pub fn is_x11(&self) -> bool {
        matches!(self, Self::X11(_))
    }

    pub fn is_wayland(&self) -> bool {
        matches!(self, Self::Xdg(_))
    }

    pub fn wl_surface(&self) -> Option<WlSurface> {
        match self {
            Self::Xdg(w) => w.wl_surface(),
            Self::X11(w) => w.wl_surface(),
        }
    }

    pub fn user_data(&self) -> &UserDataMap {
        match self {
            Self::Xdg(w) => w.user_data(),
            Self::X11(w) => w.user_data(),
        }
    }
}

impl IsAlive for WindowElement {
    fn alive(&self) -> bool {
        match self {
            Self::Xdg(w) => w.alive(),
            Self::X11(w) => w.alive(),
        }
    }
}

impl PointerTarget<NoWayState> for WindowElement {
    fn enter(&self, seat: &Seat<NoWayState>, data: &mut NoWayState, event: &MotionEvent) {
        match self {
            Self::Xdg(w) => PointerTarget::enter(w, seat, data, event),
            Self::X11(w) => PointerTarget::enter(w, seat, data, event),
        }
    }

    fn motion(&self, seat: &Seat<NoWayState>, data: &mut NoWayState, event: &MotionEvent) {
        match self {
            Self::Xdg(w) => PointerTarget::motion(w, seat, data, event),
            Self::X11(w) => PointerTarget::motion(w, seat, data, event),
        }
    }

    fn relative_motion(
        &self,
        seat: &Seat<NoWayState>,
        data: &mut NoWayState,
        event: &RelativeMotionEvent,
    ) {
        match self {
            Self::Xdg(w) => PointerTarget::relative_motion(w, seat, data, event),
            Self::X11(w) => PointerTarget::relative_motion(w, seat, data, event),
        }
    }

    fn button(&self, seat: &Seat<NoWayState>, data: &mut NoWayState, event: &ButtonEvent) {
        match self {
            Self::Xdg(w) => PointerTarget::button(w, seat, data, event),
            Self::X11(w) => PointerTarget::button(w, seat, data, event),
        }
    }

    fn axis(&self, seat: &Seat<NoWayState>, data: &mut NoWayState, frame: AxisFrame) {
        match self {
            Self::Xdg(w) => PointerTarget::axis(w, seat, data, frame),
            Self::X11(w) => PointerTarget::axis(w, seat, data, frame),
        }
    }

    fn leave(&self, seat: &Seat<NoWayState>, data: &mut NoWayState, serial: Serial, time: u32) {
        match self {
            Self::Xdg(w) => PointerTarget::leave(w, seat, data, serial, time),
            Self::X11(w) => PointerTarget::leave(w, seat, data, serial, time),
        }
    }
}

impl KeyboardTarget<NoWayState> for WindowElement {
    fn enter(
        &self,
        seat: &Seat<NoWayState>,
        data: &mut NoWayState,
        keys: Vec<KeysymHandle<'_>>,
        serial: Serial,
    ) {
        match self {
            Self::Xdg(w) => KeyboardTarget::enter(w, seat, data, keys, serial),
            Self::X11(w) => KeyboardTarget::enter(w, seat, data, keys, serial),
        }
    }

    fn leave(&self, seat: &Seat<NoWayState>, data: &mut NoWayState, serial: Serial) {
        match self {
            Self::Xdg(w) => KeyboardTarget::leave(w, seat, data, serial),
            Self::X11(w) => KeyboardTarget::leave(w, seat, data, serial),
        }
    }

    fn key(
        &self,
        seat: &Seat<NoWayState>,
        data: &mut NoWayState,
        key: KeysymHandle<'_>,
        state: KeyState,
        serial: Serial,
        time: u32,
    ) {
        match self {
            Self::Xdg(w) => KeyboardTarget::key(w, seat, data, key, state, serial, time),
            Self::X11(w) => KeyboardTarget::key(w, seat, data, key, state, serial, time),
        }
    }

    fn modifiers(
        &self,
        seat: &Seat<NoWayState>,
        data: &mut NoWayState,
        modifiers: ModifiersState,
        serial: Serial,
    ) {
        match self {
            Self::Xdg(w) => KeyboardTarget::modifiers(w, seat, data, modifiers, serial),
            Self::X11(w) => KeyboardTarget::modifiers(w, seat, data, modifiers, serial),
        }
    }
}

impl SpaceElement for WindowElement {
    fn geometry(&self) -> Rectangle<i32, Logical> {
        match self {
            Self::Xdg(w) => w.geometry(),
            Self::X11(w) => w.geometry(),
        }
    }

    fn bbox(&self) -> Rectangle<i32, Logical> {
        match self {
            Self::Xdg(w) => w.bbox(),
            Self::X11(w) => w.bbox(),
        }
    }

    fn is_in_input_region(&self, point: &Point<f64, Logical>) -> bool {
        match self {
            Self::Xdg(w) => w.is_in_input_region(point),
            Self::X11(w) => w.is_in_input_region(point),
        }
    }

    fn set_activate(&self, activated: bool) {
        match self {
            Self::Xdg(w) => w.set_activate(activated),
            Self::X11(w) => w.set_activate(activated),
        }
    }

    fn output_enter(&self, output: &Output, overlap: Rectangle<i32, Logical>) {
        match self {
            Self::Xdg(w) => w.output_enter(output, overlap),
            Self::X11(w) => w.output_enter(output, overlap),
        }
    }

    fn output_leave(&self, output: &Output) {
        match self {
            Self::Xdg(w) => w.output_leave(output),
            Self::X11(w) => w.output_leave(output),
        }
    }
}

render_elements! {
    pub WindowRenderElement<R>
        where R: ImportAll + ImportMem;
    Window=WaylandSurfaceRenderElement<R>,
    Decoration=SolidColorRenderElement,
}

impl<R: Renderer + std::fmt::Debug> std::fmt::Debug for WindowRenderElement<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::Decoration(arg0) => f.debug_tuple("Decoration").field(arg0).finish(),
            Self::_GenericCatcher(arg0) => f.debug_tuple("_GenericCatcher").field(arg0).finish(),
        }
    }
}

impl<R> AsRenderElements<R> for WindowElement
where
    R: Renderer + ImportAll + ImportMem,
    <R as Renderer>::TextureId: Texture + 'static,
{
    type RenderElement = WindowRenderElement<R>;

    fn render_elements<C: From<Self::RenderElement>>(
        &self,
        renderer: &mut R,
        location: Point<i32, Physical>,
        scale: Scale<f64>,
    ) -> Vec<C> {
        match self {
            Self::Xdg(xdg) => AsRenderElements::<R>::render_elements::<WindowRenderElement<R>>(
                xdg, renderer, location, scale,
            ),
            Self::X11(x11) => AsRenderElements::<R>::render_elements::<WindowRenderElement<R>>(
                x11, renderer, location, scale,
            ),
        }
        .into_iter()
        .map(C::from)
        .collect()
    }
}
