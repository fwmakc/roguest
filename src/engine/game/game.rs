use std::thread;
use std::time::{Duration, Instant};

use crate::eco::entities::creature::Creature;
use crate::engine::GameScene;
use crate::engine::inputs::InputHandler;

pub struct Game {
    pub player: Option<Creature>,
    pub scenes: Vec<Box<dyn GameScene>>,
    pub input: InputHandler,

    pub target_fps: f64,
    last_frame: Instant,

    worked: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: None,
            scenes: Vec::new(),
            input: InputHandler::new(10),

            last_frame: Instant::now(),
            target_fps: 60.0,

            worked: true,
        }
    }

    pub fn set_player(&mut self, player: Creature) {
        self.player = Some(player);
    }

    pub fn set_fps(&mut self, fps: f64) {
        self.target_fps = fps;
    }

    // Метод для добавления сцены в список
    pub fn add_scene<S: GameScene + 'static>(&mut self, mut scene: S) {
        scene.mounted(self);
        self.scenes.push(Box::new(scene));
    }

    // Поиск сцены, например для активации или деактивации из другой сцены
    // if let Some(scene) = game.find_scene("scene_name") {
    //   println!("{}", scene.base().get_name());
    //   scene.activate();
    // }
    pub fn find_scene(&mut self, name: &str) -> Option<&mut Box<dyn GameScene>> {
        self.scenes.iter_mut().find(|s| s.base().get_name() == name)
    }

    // Тот самый gameloop, переделанный в метод структуры
    pub fn run(&mut self) {
        let target_duration = Duration::from_secs_f64(1.0 / self.target_fps);

        self.last_frame = Instant::now();

        while self.worked {
            let start_time = Instant::now(); // Время начала кадра
            let delta_time = start_time.duration_since(self.last_frame);
            self.last_frame = start_time;

            // Временно забираем сцены, чтобы Game не был "заблокирован" заимствованием
            // let mut scenes = std::mem::take(&mut self.scenes);

            for i in 0..self.scenes.len() {
                // for scene in scenes.iter_mut() {
                let mut scene = self.scenes.remove(i);

                // Если сцена активна — запускаем логику
                if scene.base().is_active() {
                    scene.update(self, delta_time);
                }

                // Если игра остановлена, прекращаем работу всего движка
                if !self.worked {
                    break;
                }

                // Если сцена еще активна — запускаем рендеринг
                if scene.base().is_active() {
                    scene.rendering(self);
                }

                // Возвращаем сцену на её законное место
                self.scenes.insert(i, scene);
            }

            // self.scenes = scenes;

            // Контроль FPS: Задержка (Sleep)
            let frame_duration = start_time.elapsed(); // Сколько времени ушло на логику
            if frame_duration < target_duration {
                // Спим разницу между целевым временем и затраченным
                thread::sleep(target_duration - frame_duration);
            }

            // Если все сцены стали неактивны, можно тоже завершить цикл
            if self.scenes.iter().all(|s| !s.base().is_active()) {
                break;
            }
        }
    }

    pub fn stop(&mut self) {
        self.worked = false;
    }
}
