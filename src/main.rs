use std::io;

mod eco;

mod engine;
use engine::Game;

mod scenes;

// TODO: сделать...
// - таверну с пивом, где можно тратить полученные деньги
// - город, откуда можно идти в таверну или в лес мочить гоблинов
// - вывод текстовой графики на экран консоли с управлением клавишами за человечка
// - выбор в консоли
// - нормальный графический вывод в окошко винды

fn main() -> io::Result<()> {
    let mut game = Game::new();

    game.add_scene(scenes::TitleScene::new());
    game.add_scene(scenes::BattleScene::new());
    game.run();

    println!("Игра завершена");

    Ok(())
}
