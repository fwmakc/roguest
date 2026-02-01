use roguest::graphics::{renderer::Renderer, window::GraphicsWindow};

fn main() {
    // Создаем графическое окно
    let graphics_window = pollster::block_on(GraphicsWindow::new("Roguest Graphics Test"));

    // Извлекаем окно для передачи в рендерер
    let window = graphics_window.window;

    // Создаем рендерер
    let renderer = pollster::block_on(Renderer::new(&window));

    println!("Графическая система успешно инициализирована!");
    println!(
        "Размер окна: {}x{}",
        renderer.config.width, renderer.config.height
    );

    // Здесь в будущем будет основной игровой цикл
    // Пока просто выведем сообщение и завершим программу
    drop(renderer);
}
