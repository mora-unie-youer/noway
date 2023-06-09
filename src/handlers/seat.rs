use smithay::{
    backend::input::{
        AbsolutePositionEvent, Axis, AxisSource, ButtonState, Event, InputBackend, InputEvent,
        KeyboardKeyEvent, PointerAxisEvent, PointerButtonEvent,
    },
    delegate_seat,
    input::{
        keyboard::FilterResult,
        pointer::{AxisFrame, ButtonEvent, CursorImageStatus, MotionEvent},
        Seat, SeatHandler, SeatState,
    },
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::SERIAL_COUNTER,
};

use crate::{render::window::WindowElement, state::NoWayState};

impl NoWayState {
    pub fn process_input_event<I: InputBackend>(&mut self, event: InputEvent<I>) {
        match event {
            InputEvent::Keyboard { event, .. } => {
                let serial = SERIAL_COUNTER.next_serial();
                let time = Event::time_msec(&event);

                self.seat.get_keyboard().unwrap().input::<(), _>(
                    self,
                    event.key_code(),
                    event.state(),
                    serial,
                    time,
                    |_, _, _| FilterResult::Forward,
                );
            }
            InputEvent::PointerMotion { .. } => {}
            InputEvent::PointerMotionAbsolute { event, .. } => {
                let output = self.space.outputs().next().unwrap();
                let output_geo = self.space.output_geometry(output).unwrap();
                let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();

                let serial = SERIAL_COUNTER.next_serial();
                let pointer = self.seat.get_pointer().unwrap();
                let focus = self.surface_under_pointer(&pointer);

                pointer.motion(
                    self,
                    focus,
                    &MotionEvent {
                        location: pos,
                        serial,
                        time: event.time_msec(),
                    },
                );

                self.pointer_location = pointer.current_location();
            }
            InputEvent::PointerButton { event, .. } => {
                let pointer = self.seat.get_pointer().unwrap();
                let keyboard = self.seat.get_keyboard().unwrap();

                let serial = SERIAL_COUNTER.next_serial();
                let button = event.button_code();
                let button_state = event.state();

                if ButtonState::Pressed == button_state && !pointer.is_grabbed() {
                    if let Some((window, _)) = self.window_under_pointer(&pointer) {
                        let window = window.clone();
                        self.space.raise_element(&window, true);
                        self.space.elements().for_each(|window| {
                            if let WindowElement::Xdg(window) = window {
                                window.toplevel().send_configure();
                            }
                        });

                        keyboard.set_focus(self, Some(window.wl_surface().unwrap()), serial);
                    } else {
                        self.space.elements().for_each(|window| {
                            if let WindowElement::Xdg(window) = window {
                                window.set_activated(false);
                                window.toplevel().send_configure();
                            }
                        });

                        keyboard.set_focus(self, None, serial);
                    }
                };

                pointer.button(
                    self,
                    &ButtonEvent {
                        button,
                        state: button_state,
                        serial,
                        time: event.time_msec(),
                    },
                );
            }
            InputEvent::PointerAxis { event, .. } => {
                let source = event.source();

                let horizontal_amount = event
                    .amount(Axis::Horizontal)
                    .unwrap_or_else(|| event.amount_discrete(Axis::Horizontal).unwrap() * 3.0);
                let vertical_amount = event
                    .amount(Axis::Vertical)
                    .unwrap_or_else(|| event.amount_discrete(Axis::Vertical).unwrap() * 3.0);
                let horizontal_amount_discrete = event.amount_discrete(Axis::Horizontal);
                let vertical_amount_discrete = event.amount_discrete(Axis::Vertical);

                let mut frame = AxisFrame::new(event.time_msec()).source(source);

                if horizontal_amount != 0.0 {
                    frame = frame.value(Axis::Horizontal, horizontal_amount);
                    if let Some(discrete) = horizontal_amount_discrete {
                        frame = frame.discrete(Axis::Horizontal, discrete as i32);
                    }
                } else if source == AxisSource::Finger {
                    frame = frame.stop(Axis::Horizontal);
                }

                if vertical_amount != 0.0 {
                    frame = frame.value(Axis::Vertical, vertical_amount);
                    if let Some(discrete) = vertical_amount_discrete {
                        frame = frame.discrete(Axis::Vertical, discrete as i32);
                    }
                } else if source == AxisSource::Finger {
                    frame = frame.stop(Axis::Vertical);
                }

                self.seat.get_pointer().unwrap().axis(self, frame);
            }
            _ => {}
        }
    }
}

impl SeatHandler for NoWayState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn cursor_image(&mut self, _seat: &Seat<Self>, image: CursorImageStatus) {
        *self.cursor_status.lock().unwrap() = image;
    }
}

delegate_seat!(NoWayState);
