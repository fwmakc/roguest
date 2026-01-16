use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{
    eco::entities::creature::Creature,
    engine::{GameScene, gameloop, inputs::InputHandler},
};

pub struct Game {
    pub player: Option<Creature>,
    pub scenes: Vec<Box<dyn GameScene>>,
    pub input: InputHandler,

    pub target_fps: f64,
    pub last_frame: Instant,

    pub worked: bool,
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
        gameloop(self);
    }

    pub fn stop(&mut self) {
        self.worked = false;
    }
}
