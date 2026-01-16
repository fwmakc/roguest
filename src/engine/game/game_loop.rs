use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{
    eco::entities::creature::Creature,
    engine::{Game, GameScene, inputs::InputHandler},
};

// Тот самый gameloop, переделанный в метод структуры
pub fn gameloop(game: &mut Game) {
    let target_duration = Duration::from_secs_f64(1.0 / game.target_fps);

    game.last_frame = Instant::now();

    while game.worked {
        let start_time = Instant::now(); // Время начала кадра
        let delta_time = start_time.duration_since(game.last_frame);
        game.last_frame = start_time;

        // Временно забираем сцены, чтобы Game не был "заблокирован" заимствованием
        // let mut scenes = std::mem::take(&mut game.scenes);

        for i in 0..game.scenes.len() {
            // for scene in scenes.iter_mut() {
            let mut scene = game.scenes.remove(i);

            // Если сцена активна — запускаем логику
            if scene.base().is_active() {
                scene.update(game, delta_time);
            }

            // Если игра остановлена, прекращаем работу всего движка
            if !game.worked {
                break;
            }

            // Если сцена еще активна — запускаем рендеринг
            if scene.base().is_active() {
                scene.rendering(game);
            }

            // Возвращаем сцену на её законное место
            game.scenes.insert(i, scene);

            // Если игра остановлена, прекращаем работу всего движка
            if !game.worked {
                break;
            }
        }

        // game.scenes = scenes;

        // Контроль FPS: Задержка (Sleep)
        let frame_duration = start_time.elapsed(); // Сколько времени ушло на логику
        if frame_duration < target_duration {
            // Спим разницу между целевым временем и затраченным
            thread::sleep(target_duration - frame_duration);
        }

        // Если все сцены стали неактивны, можно тоже завершить цикл
        if game.scenes.iter().all(|s| !s.base().is_active()) {
            break;
        }
    }
}
