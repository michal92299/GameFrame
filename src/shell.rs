use smithay::{
    desktop::{Space, Window},
    wayland::{
        compositor::{CompositorHandler, SurfaceData},
        shell::xdg::{XdgShellHandler, XdgToplevelSurfaceData},
    },
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Point, Logical},
};

use crate::state::State;

impl CompositorHandler for State {
    fn surface_commit(&mut self, surface: &WlSurface) {
        // Commit logic
    }
}

impl XdgShellHandler for State {
    fn new_toplevel_surface(&mut self, surface: &smithay::wayland::shell::xdg::ToplevelSurface) {
        let window = Window::new(surface.clone());
        self.space.map_window(&window, Point::from((0, 0)), true);
    }
}

pub fn handle_pointer_motion(state: &mut State, position: Point<f64, Logical>) {
    // Update cursor position, focus, etc.
    if let Some(surface) = state.space.window_under(position).cloned() {
        state.pointer.motion(position, Some((&surface, Point::from((0.0, 0.0)))));
    }
}
