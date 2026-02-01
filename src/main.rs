use std::io;

mod objects;

mod engine;
use engine::Game;

use crate::interface::prints;

mod graphics;
mod interface;
mod scenes;

// TODO: сделать...
// - таверну с пивом, где можно тратить полученные деньги
// - вывод текстовой графики на экран консоли с управлением клавишами за человечка
// - нормальный графический вывод в окошко винды

fn main() -> io::Result<()> {
    // Инициализация графической системы (в будущем)
    // let graphics_window = graphics::window::GraphicsWindow::new("Roguest").await;
    // let window = graphics_window.window;
    // let renderer = graphics::renderer::Renderer::new(window).await;

    let mut game = Game::new();

    game.set_fps(60.0);
    game.set_scenes_len(10);

    game.add_scene(scenes::TownScene::new());
    game.add_scene(scenes::TavernScene::new());
    // game.add_scene(scenes::BattleScene::new());

    game.add_scene(scenes::TitleScene::new());

    game.run();

    prints::message("Игра завершена");

    Ok(())
}
