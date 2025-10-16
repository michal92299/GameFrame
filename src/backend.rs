use smithay::{
    backend::{
        allocator::gbm::{GbmAllocator, GbmDevice},
        drm::{DrmBackend, DrmDevice, DrmEvent, DrmSurface},
        session::{libseat::LibSeatSession, Session, Signal as SessionSignal},
        udev::{UdevBackend, UdevEvent},
    },
    reexports::calloop::{Dispatcher, EventLoop, RegistrationToken},
    utils::DeviceFd,
};
use slog::{info, Logger};
use std::sync::{Arc, Mutex};

use crate::state::State;
use crate::renderer::{init_renderer, RendererType};

pub fn init_backend(
    event_loop: &mut EventLoop<'static, State>,
    logger: Logger,
) -> anyhow::Result<(LibSeatSession, RegistrationToken)> {
    let (session, notifier) = LibSeatSession::new()?;

    let udev_backend = UdevBackend::new(session.clone(), logger.clone())?;

    let udev_dispatcher = Dispatcher::new(udev_backend, move |event, _, state: &mut State| {
        match event {
            UdevEvent::Added { device_id, path, seat, .. } => {
                info!(state.log, "New device added: {:?}", path);
                if let Err(e) = setup_drm_device(state, device_id, &session) {
                    info!(state.log, "Error setting up DRM: {}", e);
                }
            }
            _ => {}
        }
    });

    let token = event_loop.handle().register_dispatcher(udev_dispatcher)?;

    let session_dispatcher = Dispatcher::new(notifier, |signal, _, state: &mut State| {
        match signal {
            SessionSignal::ActivateSession { .. } => {
                info!(state.log, "Session activated");
            }
            _ => {}
        }
    });
    event_loop.handle().register_dispatcher(session_dispatcher)?;

    Ok((session, token))
}

fn setup_drm_device(state: &mut State, device_id: u64, session: &LibSeatSession) -> anyhow::Result<()> {
    let fd = session.device_fd(device_id)?;
    let drm_device = DrmDevice::new(fd, true, state.log.clone())?;
    let gbm_device = GbmDevice::new(DeviceFd::from(fd.clone()))?;

    let renderer_type = if state.config.use_vulkan && cfg!(feature = "vulkan") {
        RendererType::Vulkan
    } else {
        RendererType::OpenGL
    };

    let renderer = init_renderer(gbm_device.clone(), renderer_type, state.log.clone())?;

    let allocator = GbmAllocator::new(gbm_device, smithay::backend::allocator::gbm::GbmBufferFlags::RENDERING);

    let surface = drm_device.create_surface(
        &state.connectors[0],  // Assume first connector
        (1920, 1080),  // Default resolution
        vec![/* formats */],
    )?;

    state.drm_surface = Some(Arc::new(Mutex::new(surface)));
    state.renderer = Some(renderer);

    Ok(())
}
