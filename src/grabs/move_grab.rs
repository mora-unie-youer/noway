use smithay::{
    input::pointer::{
        AxisFrame, ButtonEvent, GrabStartData, MotionEvent, PointerGrab, PointerInnerHandle,
        RelativeMotionEvent,
    },
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Point},
};

use crate::{render::window::WindowElement, state::NoWayState};

pub struct MoveSurfaceGrab {
    pub start_data: GrabStartData<NoWayState>,
    pub window: WindowElement,
    pub initial_window_location: Point<i32, Logical>,
}

impl PointerGrab<NoWayState> for MoveSurfaceGrab {
    fn motion(
        &mut self,
        data: &mut NoWayState,
        handle: &mut PointerInnerHandle<'_, NoWayState>,
        _focus: Option<(WlSurface, Point<i32, Logical>)>,
        event: &MotionEvent,
    ) {
        // While the grab is active, no client has pointer focus
        handle.motion(data, None, event);

        let delta = event.location - self.start_data.location;
        let new_location = self.initial_window_location.to_f64() + delta;
        data.space
            .map_element(self.window.clone(), new_location.to_i32_round(), true);
    }

    fn relative_motion(
        &mut self,
        data: &mut NoWayState,
        handle: &mut PointerInnerHandle<'_, NoWayState>,
        focus: Option<(WlSurface, Point<i32, Logical>)>,
        event: &RelativeMotionEvent,
    ) {
        handle.relative_motion(data, focus, event);
    }

    fn button(
        &mut self,
        data: &mut NoWayState,
        handle: &mut PointerInnerHandle<'_, NoWayState>,
        event: &ButtonEvent,
    ) {
        handle.button(data, event);

        // The button is a button code as defined in the
        // Linux kernel's linux/input-event-codes.h header file, e.g. BTN_LEFT.
        const BTN_LEFT: u32 = 0x110;

        if !handle.current_pressed().contains(&BTN_LEFT) {
            // No more buttons are pressed, release the grab.
            handle.unset_grab(data, event.serial, event.time);
        }
    }

    fn axis(
        &mut self,
        data: &mut NoWayState,
        handle: &mut PointerInnerHandle<'_, NoWayState>,
        details: AxisFrame,
    ) {
        handle.axis(data, details)
    }

    fn start_data(&self) -> &GrabStartData<NoWayState> {
        &self.start_data
    }
}
