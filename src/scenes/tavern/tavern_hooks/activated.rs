use crate::{
    engine::{Game, Scene},
    scenes::TavernScene,
};

pub fn activated(scene: &mut TavernScene) {
    println!("{}", "Вы находитесь в таверне!");
}
