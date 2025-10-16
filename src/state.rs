use anyhow::Result;
use smithay::{
    backend::renderer::Renderer,
    desktop::Space,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Clock, Monotonic},
    wayland::{
        compositor::CompositorState,
        seat::{KeyboardHandle, PointerHandle, SeatState},
    },
    xwayland::XWayland,
};
use slog::Logger;
use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::renderer::MultiRenderer;

pub struct State {
    pub display_handle: DisplayHandle,
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub seat_state: SeatState<Self>,
    pub output_manager_state: OutputManagerState,
    pub input_manager_state: InputManagerState,
    pub data_device_state: DataDeviceState,
    pub dmabuf_state: DmabufState,
    pub fractional_scale_manager_state: FractionalScaleManagerState,
    pub viewporter_state: ViewporterState,
    pub presentation_state: PresentationState,
    pub space: Space<WlSurface>,
    pub pointer: PointerHandle<Self>,
    pub keyboard: KeyboardHandle<Self>,
    pub clock: Clock<Monotonic>,
    pub log: Logger,
    pub xwayland: Option<XWayland>,
    pub outputs: Vec<smithay::output::Output>,
    pub drm_surface: Option<Arc<Mutex<DrmSurface>>>,
    pub renderer: Option<MultiRenderer<'static>>,
    pub config: Config,
    pub running: bool,
}

impl State {
    pub fn new(dh: &DisplayHandle, config: Config, log: Logger) -> Self {
        Self {
            display_handle: dh.clone(),
            compositor_state: CompositorState::new::<Self>(dh),
            xdg_shell_state: XdgShellState::new::<Self>(dh),
            shm_state: ShmState::new::<Self>(dh, vec![]),
            seat_state: SeatState::new(),
            output_manager_state: OutputManagerState::new_with_xdg_output::<Self>(dh),
            input_manager_state: InputManagerState::new::<Self>(dh),
            data_device_state: DataDeviceState::new::<Self>(dh),
            dmabuf_state: DmabufState::new(),
            fractional_scale_manager_state: FractionalScaleManagerState::new::<Self>(dh),
            viewporter_state: ViewporterState::new::<Self>(dh),
            presentation_state: PresentationState::new::<Self>(dh, 1000),
            space: Space::default(),
            pointer: PointerHandle::<Self>::new(),  // Will be set later
            keyboard: KeyboardHandle::<Self>::new(),  // Will be set later
            clock: Clock::new().expect("Failed to create clock"),
            log,
            xwayland: None,
            outputs: vec![],
            drm_surface: None,
            renderer: None,
            config,
            running: true,
        }
    }

    pub fn render(&mut self) -> Result<()> {
        if let (Some(surface), Some(renderer)) = (&self.drm_surface, &mut self.renderer) {
            let surface = surface.lock().unwrap();
            renderer.render(|_tex| {
                // Rendering logic for space
                self.space.render(&mut *renderer, (0, 0))?;
                Ok(())
            })?;
            surface.commit()?;
        }
        Ok(())
    }
}
