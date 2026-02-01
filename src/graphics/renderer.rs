use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureView};
use winit::window::Window;

// Обертка над Surface, которая позволяет обойти ограничения lifetime
pub struct SurfaceWrapper {
    surface: Surface<'static>,
}

impl SurfaceWrapper {
    pub fn new(surface: Surface<'static>) -> Self {
        Self { surface }
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    pub fn configure(&self, device: &Device, config: &SurfaceConfiguration) {
        self.surface.configure(device, config);
    }

    pub fn get_capabilities(&self, adapter: &wgpu::Adapter) -> wgpu::SurfaceCapabilities {
        self.surface.get_capabilities(adapter)
    }
}

pub struct Renderer {
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    surface_wrapper: SurfaceWrapper,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        // Создаем instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        // Получаем surface из окна
        let surface = unsafe {
            instance
                .create_surface(window)
                .expect("Failed to create surface")
        };

        // Получаем adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Создаем device и queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Получаем размеры окна
        let size = window.inner_size();

        // Определяем формат поверхности
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // Создаем конфигурацию поверхности
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Создаем wrapper для surface с 'static lifetime
        let surface_wrapper =
            unsafe { std::mem::transmute::<Surface<'_>, Surface<'static>>(surface) };
        let surface_wrapper = SurfaceWrapper::new(surface_wrapper);

        Self {
            surface_wrapper,
            device,
            queue,
            config,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface_wrapper.configure(&self.device, &self.config);
        }
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface_wrapper.get_current_texture()
    }

    pub fn get_texture_view(&self, texture: &wgpu::SurfaceTexture) -> TextureView {
        texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn submit(&self, command_buffers: Vec<wgpu::CommandBuffer>) {
        // Передаем владение вектором командных буферов
        self.queue.submit(command_buffers);
    }

    pub fn present(&self, texture: wgpu::SurfaceTexture) {
        texture.present();
    }
}
