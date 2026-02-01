use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct GraphicsWindow {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl GraphicsWindow {
    pub async fn new(title: &str) -> Self {
        // Инициализируем логирование
        env_logger::init();

        // Создаем event loop
        let event_loop = EventLoop::new().unwrap();

        // Создаем окно
        let window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_title(title)
                    .with_inner_size(winit::dpi::LogicalSize::new(800, 600)),
            )
            .unwrap();

        Self { event_loop, window }
    }
}
