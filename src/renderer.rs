use smithay::backend::renderer::{
    gles::GlesRenderer,
    multigpu::{MultiRenderer, MultiTexture},
    vulkan::VulkanRenderer,
};
use gbm::Device as GbmDevice;
use slog::Logger;
use anyhow::Result;

#[derive(Debug, Copy, Clone)]
pub enum RendererType {
    OpenGL,
    Vulkan,
}

pub fn init_renderer(gbm: GbmDevice<DeviceFd>, ty: RendererType, log: Logger) -> Result<MultiRenderer<'static>> {
    match ty {
        RendererType::OpenGL => {
            let egl = unsafe { egl::EGLDisplay::new(&gbm)? };
            let renderer = unsafe { GlesRenderer::new(egl::EGLContext::new(&egl)?)? };
            Ok(MultiRenderer::Gles(renderer))
        }
        RendererType::Vulkan => {
            #[cfg(feature = "vulkan")]
            {
                let instance = ash::Instance::new(/* ... */)?;
                let renderer = VulkanRenderer::new(instance, /* physical device, etc. */)?;
                Ok(MultiRenderer::Vulkan(renderer))
            }
            #[cfg(not(feature = "vulkan"))]
            anyhow::bail!("Vulkan not enabled")
        }
    }
}
