use crate::{
    engine::{Game, Scene},
    scenes::TavernScene,
};

pub fn mounted(scene: &mut TavernScene, game: &mut Game) {
    println!("{}", "Вы находитесь в таверне!");
}
