//! Connect a window with a renderer.
use crate::core::Color;
use crate::graphics::color;
use crate::graphics::compositor;
use crate::graphics::error;
use crate::graphics::{self, Shell, Viewport};
use crate::settings::{self, Settings};
use crate::{Engine, Renderer};

/// A window graphics backend for iced powered by `wgpu`.
pub struct Compositor {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    format: wgpu::TextureFormat,
    alpha_mode: wgpu::CompositeAlphaMode,
    engine: Engine,
    settings: Settings,
}

/// A compositor error.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// The surface creation failed.
    #[error("the surface creation failed: {0}")]
    SurfaceCreationFailed(#[from] wgpu::CreateSurfaceError),
    /// The surface is not compatible.
    #[error("the surface is not compatible")]
    IncompatibleSurface,
    /// No adapter was found for the options requested.
    #[error("no adapter was found for the options requested: {0:?}")]
    NoAdapterFound(String),
    /// No device request succeeded.
    #[error("no device request succeeded: {0:?}")]
    RequestDeviceFailed(Vec<(wgpu::Limits, wgpu::RequestDeviceError)>),
}

impl From<Error> for graphics::Error {
    fn from(error: Error) -> Self {
        Self::GraphicsAdapterNotFound {
            backend: "wgpu",
            reason: error::Reason::RequestFailed(error.to_string()),
        }
    }
}

impl Compositor {
    /// Requests a new [`Compositor`] with the given [`Settings`].
    ///
    /// Returns `None` if no compatible graphics adapter could be found.
    pub async fn request<W: compositor::Window>(
        settings: Settings,
        compatible_window: Option<W>,
        shell: Shell,
    ) -> Result<Self, Error> {
        let instance = create_instance(settings).await;

        log::info!("{settings:#?}");
        log_available_adapters(&instance, settings.backends);

        let compatible_surface = create_compatible_surface(&instance, compatible_window);
        let adapter = request_adapter(&instance, compatible_surface.as_ref()).await?;

        log::info!("Selected: {:#?}", adapter.get_info());

        let (format, alpha_mode) =
            select_surface_format_and_alpha(compatible_surface.as_ref(), &adapter)?;

        log::info!("Selected format: {format:?} with alpha mode: {alpha_mode:?}");
        let engine = request_engine(&adapter, settings, format, shell).await?;

        Ok(Compositor {
            instance,
            adapter,
            format,
            alpha_mode,
            engine,
            settings,
        })
    }
}

async fn create_instance(settings: Settings) -> wgpu::Instance {
    wgpu::util::new_instance_with_webgpu_detection(&wgpu::InstanceDescriptor {
        backends: settings.backends,
        flags: instance_flags(),
        ..Default::default()
    })
    .await
}

fn instance_flags() -> wgpu::InstanceFlags {
    if cfg!(feature = "strict-assertions") {
        wgpu::InstanceFlags::debugging()
    } else {
        wgpu::InstanceFlags::empty()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn log_available_adapters(instance: &wgpu::Instance, backends: wgpu::Backends) {
    if log::max_level() < log::LevelFilter::Info {
        return;
    }
    let available_adapters: Vec<_> = instance
        .enumerate_adapters(backends)
        .iter()
        .map(wgpu::Adapter::get_info)
        .collect();
    log::info!("Available adapters: {available_adapters:#?}");
}

#[cfg(target_arch = "wasm32")]
fn log_available_adapters(_instance: &wgpu::Instance, _backends: wgpu::Backends) {}

fn create_compatible_surface<W: compositor::Window>(
    instance: &wgpu::Instance,
    compatible_window: Option<W>,
) -> Option<wgpu::Surface<'static>> {
    #[allow(unsafe_code)]
    {
        compatible_window.and_then(|window| instance.create_surface(window).ok())
    }
}

async fn request_adapter(
    instance: &wgpu::Instance,
    compatible_surface: Option<&wgpu::Surface<'static>>,
) -> Result<wgpu::Adapter, Error> {
    let adapter_options = wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::from_env()
            .unwrap_or(wgpu::PowerPreference::HighPerformance),
        compatible_surface,
        force_fallback_adapter: false,
    };
    instance
        .request_adapter(&adapter_options)
        .await
        .map_err(|_error| Error::NoAdapterFound(format!("{adapter_options:?}")))
}

fn select_surface_format_and_alpha(
    compatible_surface: Option<&wgpu::Surface<'static>>,
    adapter: &wgpu::Adapter,
) -> Result<(wgpu::TextureFormat, wgpu::CompositeAlphaMode), Error> {
    let Some(surface) = compatible_surface else {
        return Err(Error::IncompatibleSurface);
    };
    let capabilities = surface.get_capabilities(adapter);
    log::info!("Available formats: {:#?}", capabilities.formats);
    log::info!("Available alpha modes: {:#?}", capabilities.alpha_modes);
    let format = choose_surface_format(&capabilities).ok_or(Error::IncompatibleSurface)?;
    let alpha_mode = choose_alpha_mode(&capabilities.alpha_modes);
    Ok((format, alpha_mode))
}

fn choose_surface_format(capabilities: &wgpu::SurfaceCapabilities) -> Option<wgpu::TextureFormat> {
    let mut formats = capabilities
        .formats
        .iter()
        .copied()
        .filter(|format| format.required_features() == wgpu::Features::empty());
    let preferred = if color::GAMMA_CORRECTION {
        formats.find(wgpu::TextureFormat::is_srgb)
    } else {
        formats.find(|format| !wgpu::TextureFormat::is_srgb(format))
    };
    preferred.or_else(|| {
        log::warn!("No format found!");
        capabilities.formats.first().copied()
    })
}

fn choose_alpha_mode(alpha_modes: &[wgpu::CompositeAlphaMode]) -> wgpu::CompositeAlphaMode {
    if alpha_modes.contains(&wgpu::CompositeAlphaMode::PostMultiplied) {
        wgpu::CompositeAlphaMode::PostMultiplied
    } else if alpha_modes.contains(&wgpu::CompositeAlphaMode::PreMultiplied) {
        wgpu::CompositeAlphaMode::PreMultiplied
    } else {
        wgpu::CompositeAlphaMode::Auto
    }
}

async fn request_engine(
    adapter: &wgpu::Adapter,
    settings: Settings,
    format: wgpu::TextureFormat,
    shell: Shell,
) -> Result<Engine, Error> {
    let mut errors = Vec::new();
    for required_limits in request_limits(adapter) {
        match request_device(adapter, settings, &required_limits).await {
            Ok((device, queue)) => {
                return Ok(Engine::new(
                    adapter,
                    device,
                    queue,
                    format,
                    settings.antialiasing,
                    shell.clone(),
                ));
            }
            Err(error) => errors.push((required_limits, error)),
        }
    }
    Err(Error::RequestDeviceFailed(errors))
}

#[cfg(target_arch = "wasm32")]
fn request_limits(adapter: &wgpu::Adapter) -> Vec<wgpu::Limits> {
    vec![wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())]
        .into_iter()
        .map(compositor_limits)
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn request_limits(_adapter: &wgpu::Adapter) -> Vec<wgpu::Limits> {
    vec![wgpu::Limits::default(), wgpu::Limits::downlevel_defaults()]
        .into_iter()
        .map(compositor_limits)
        .collect()
}

fn compositor_limits(limits: wgpu::Limits) -> wgpu::Limits {
    wgpu::Limits {
        max_bind_groups: 2,
        max_non_sampler_bindings: 2048,
        ..limits
    }
}

async fn request_device(
    adapter: &wgpu::Adapter,
    settings: Settings,
    required_limits: &wgpu::Limits,
) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
    adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("iced_wgpu::window::compositor device descriptor"),
            required_features: settings.required_features,
            required_limits: required_limits.clone(),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            trace: wgpu::Trace::Off,
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
        })
        .await
}

/// Creates a [`Compositor`] with the given [`Settings`] and window.
pub async fn new<W: compositor::Window>(
    settings: Settings,
    compatible_window: W,
    shell: Shell,
) -> Result<Compositor, Error> {
    Compositor::request(settings, Some(compatible_window), shell).await
}

/// Presents the given primitives with the given [`Compositor`].
pub fn present(
    renderer: &mut Renderer,
    surface: &mut wgpu::Surface<'static>,
    viewport: &Viewport,
    background_color: Color,
    on_pre_present: impl FnOnce(),
) -> Result<(), compositor::SurfaceError> {
    match surface.get_current_texture() {
        Ok(frame) => {
            let view = &frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let _submission = renderer.present(
                Some(background_color),
                frame.texture.format(),
                view,
                viewport,
            );

            // Present the frame
            on_pre_present();
            frame.present();

            Ok(())
        }
        Err(error) => match error {
            wgpu::SurfaceError::Timeout => Err(compositor::SurfaceError::Timeout),
            wgpu::SurfaceError::Outdated => Err(compositor::SurfaceError::Outdated),
            wgpu::SurfaceError::Lost => Err(compositor::SurfaceError::Lost),
            wgpu::SurfaceError::OutOfMemory => Err(compositor::SurfaceError::OutOfMemory),
            wgpu::SurfaceError::Other => Err(compositor::SurfaceError::Other),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn surface_capabilities(formats: Vec<wgpu::TextureFormat>) -> wgpu::SurfaceCapabilities {
        wgpu::SurfaceCapabilities {
            formats,
            present_modes: vec![wgpu::PresentMode::AutoVsync],
            alpha_modes: vec![wgpu::CompositeAlphaMode::Opaque],
            usages: wgpu::TextureUsages::RENDER_ATTACHMENT,
        }
    }

    #[test]
    fn choose_surface_format_prefers_non_srgb_when_gamma_correction_is_disabled() {
        assert!(
            !color::GAMMA_CORRECTION,
            "test assumes default gamma correction setting"
        );

        let capabilities = surface_capabilities(vec![
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureFormat::Bgra8Unorm,
        ]);

        assert_eq!(
            choose_surface_format(&capabilities),
            Some(wgpu::TextureFormat::Bgra8Unorm),
        );
    }

    #[test]
    fn choose_alpha_mode_prefers_postmultiplied_then_premultiplied() {
        assert_eq!(
            choose_alpha_mode(&[
                wgpu::CompositeAlphaMode::Opaque,
                wgpu::CompositeAlphaMode::PreMultiplied,
                wgpu::CompositeAlphaMode::PostMultiplied,
            ]),
            wgpu::CompositeAlphaMode::PostMultiplied,
        );
        assert_eq!(
            choose_alpha_mode(&[
                wgpu::CompositeAlphaMode::Opaque,
                wgpu::CompositeAlphaMode::PreMultiplied,
            ]),
            wgpu::CompositeAlphaMode::PreMultiplied,
        );
    }
}

impl graphics::Compositor for Compositor {
    type Renderer = Renderer;
    type Surface = wgpu::Surface<'static>;

    async fn with_backend(
        settings: graphics::Settings,
        _display: impl compositor::Display,
        compatible_window: impl compositor::Window,
        shell: Shell,
        backend: Option<&str>,
    ) -> Result<Self, graphics::Error> {
        match backend {
            None | Some("wgpu") => {
                let mut settings = Settings::from(settings);

                if let Some(backends) = wgpu::Backends::from_env() {
                    settings.backends = backends;
                }

                if let Some(present_mode) = settings::present_mode_from_env() {
                    settings.present_mode = present_mode;
                }

                Ok(new(settings, compatible_window, shell).await?)
            }
            Some(backend) => Err(graphics::Error::GraphicsAdapterNotFound {
                backend: "wgpu",
                reason: error::Reason::DidNotMatch {
                    preferred_backend: backend.to_owned(),
                },
            }),
        }
    }

    fn create_renderer(&self) -> Self::Renderer {
        Renderer::new(
            self.engine.clone(),
            self.settings.default_font,
            self.settings.default_text_size,
        )
    }

    fn create_surface<W: compositor::Window>(
        &mut self,
        window: W,
        width: u32,
        height: u32,
    ) -> Self::Surface {
        let mut surface = self
            .instance
            .create_surface(window)
            .expect("Create surface");

        if width > 0 && height > 0 {
            self.configure_surface(&mut surface, width, height);
        }

        surface
    }

    fn configure_surface(&mut self, surface: &mut Self::Surface, width: u32, height: u32) {
        surface.configure(
            &self.engine.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.format,
                present_mode: self.settings.present_mode,
                width,
                height,
                alpha_mode: self.alpha_mode,
                view_formats: vec![],
                desired_maximum_frame_latency: 1,
            },
        );
    }

    fn information(&self) -> compositor::Information {
        let information = self.adapter.get_info();

        compositor::Information {
            adapter: information.name,
            backend: format!("{:?}", information.backend),
        }
    }

    fn present(
        &mut self,
        renderer: &mut Self::Renderer,
        surface: &mut Self::Surface,
        viewport: &Viewport,
        background_color: Color,
        on_pre_present: impl FnOnce(),
    ) -> Result<(), compositor::SurfaceError> {
        present(
            renderer,
            surface,
            viewport,
            background_color,
            on_pre_present,
        )
    }

    fn screenshot(
        &mut self,
        renderer: &mut Self::Renderer,
        viewport: &Viewport,
        background_color: Color,
    ) -> Vec<u8> {
        renderer.screenshot(viewport, background_color)
    }
}
