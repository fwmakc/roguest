use crate::{
    engine::{Game, Scene},
    scenes::TownScene,
};

pub fn activated(scene: &mut TownScene) {
    println!("{}", "Вы находитесь в городе!");
}
