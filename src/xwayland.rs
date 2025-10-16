use smithay::xwayland::{XWayland, XWaylandEvent, XWm};
use slog::Logger;
use anyhow::Result;

use crate::state::State;

pub fn init_xwayland(state: &mut State, event_loop: &mut calloop::EventLoop<State>) -> Result<()> {
    let (xwayland, source) = XWayland::new(state.log.clone());

    let handle = event_loop.handle();
    handle.insert_source(source, |event, _, state| {
        match event {
            XWaylandEvent::Ready { connection, client } => {
                let wm = XWm::start_wm(
                    state.display_handle.clone(),
                    connection,
                    client,
                    state.space.clone(),
                    state.log.clone(),
                ).unwrap();
                state.xwm = Some(wm);
            }
            XWaylandEvent::XServerReady { connection } => {
                // Handle X server
            }
            _ => {}
        }
    })?;

    xwayland.start(
        &state.display_handle,
        None,
        &handle,
        None,
    )?;

    state.xwayland = Some(xwayland);

    Ok(())
}
