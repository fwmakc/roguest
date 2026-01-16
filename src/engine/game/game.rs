use std::{thread, time::Instant};

use crate::{
    engine::{gameloop, scene::Scene},
    etv::types::Creature,
    interface::inputs::InputHandler,
};

pub struct Game {
    pub player: Option<Creature>,
    pub scenes: Vec<Box<dyn Scene>>,
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
    pub fn add_scene<S: Scene + 'static>(&mut self, mut scene: S) {
        scene.mounted(self);
        self.scenes.push(Box::new(scene));
    }

    // Поиск сцены, например для активации или деактивации из другой сцены
    pub fn find_scene(&mut self, name: &str) -> Option<&mut Box<dyn Scene>> {
        self.scenes.iter_mut().find(|scene| scene.name() == name)
    }

    // Удаляет сцену по имени
    pub fn remove_scene(&mut self, name: &str) {
        let index = self.scenes.iter().position(|s| s.name() == name);
        if let Some(i) = index {
            let mut scene = self.scenes.remove(i);
            scene.unmounted(self);
        }
    }

    // Тот самый gameloop, переделанный в метод структуры
    pub fn run(&mut self) {
        gameloop(self);
    }

    pub fn stop(&mut self) {
        self.worked = false;
    }
}
