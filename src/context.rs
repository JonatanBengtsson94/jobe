use winit::window::Window;

pub struct Context<'window> {
    surface: wgpu::Surface<'window>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
}

impl<'window> Context<'window> {
    async fn new(window: &'window Window) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window)
            .expect("Failed to create surface");
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance
            .request_adapter(&adapter_descriptor)
            .await
            .expect("Failed to find adapter");
        let device_descriptor = wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await
            .expect("Failed to get device");
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let surface_config = surface
            .get_default_config(&adapter, width, height)
            .expect("Surface not supported by adapter");
        surface.configure(&device, &surface_config);

        Self {
            surface,
            adapter,
            device,
            queue,
            surface_config,
        }
    }
}
