use anyhow::Result;
use calloop::{EventLoop, LoopHandle};
use slog::{info, Logger};
use smithay::{
    reexports::wayland_server::{Display, DisplayHandle},
    wayland::{
        compositor::CompositorState,
        data_device::DataDeviceState,
        dmabuf::DmabufState,
        fractional_scale::FractionalScaleManagerState,
        input::InputManagerState,
        output::OutputManagerState,
        presentation::PresentationState,
        seat::SeatState,
        shell::xdg::XdgShellState,
        shm::ShmState,
        viewporter::ViewporterState,
    },
};

use crate::backend::init_backend;
use crate::config::Config;
use crate::input::init_input;
use crate::state::State;
use crate::utils::Clock;
use crate::xwayland::init_xwayland;

pub fn run_compositor(config: &mut Config, log: Logger) -> Result<()> {
    let mut display: Display<State> = Display::new()?;
    let dh = display.handle();

    let mut state = State::new(&dh, config.clone(), log.clone());

    let mut event_loop: EventLoop<State> = EventLoop::try_new()?;

    let (_session, _token) = init_backend(&mut event_loop, log.clone())?;

    init_xwayland(&mut state, &mut event_loop)?;

    init_input(&mut state);

    state.space.map_output(&state.outputs[0], (0, 0));

    let loop_handle = event_loop.handle();

    loop {
        event_loop.dispatch(None, &mut state)?;

        display.flush_clients()?;

        state.render()?;  // Implement rendering loop with FPS limit

        if !state.running {
            break;
        }
    }

    Ok(())
}
