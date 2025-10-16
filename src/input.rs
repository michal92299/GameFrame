use smithay::{
    input::{
        Seat, SeatState, PointerHandle, KeyboardHandle, 
        pointer::{PointerEvent, PointerEventKind},
        keyboard::{KeyboardEvent, KeyEvent, ModifiersState},
    },
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::seat::{self, WaylandFocus},
};
use slog::Logger;

use crate::state::State;
use crate::shell::handle_pointer_motion;

pub fn init_input(state: &mut State) {
    let seat_name = "seat0";
    let mut seat = state.seat_state.new_seat(seat_name);

    state.pointer = seat.add_pointer();
    state.keyboard = seat.add_keyboard(Default::default(), 200, 25)?;
}

pub fn process_input(state: &mut State, event: input::Event) {
    match event {
        input::Event::Pointer { event, .. } => match event {
            PointerEvent::Motion { event } => {
                handle_pointer_motion(state, event.position);
            }
            _ => {}
        },
        input::Event::Keyboard { event } => {
            // Handle key presses, e.g., exit on ESC
            if let KeyEvent { key, state: key_state, .. } = event {
                if key == 1 && key_state == seat::KeyState::Pressed {  // ESC
                    state.running = false;
                }
            }
        }
        _ => {}
    }
}
