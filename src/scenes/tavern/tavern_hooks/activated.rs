use crate::{
    engine::{Game, Scene},
    interface::prints,
    scenes::TavernScene,
};

pub fn activated(scene: &mut TavernScene) {
    prints::message("Вы находитесь в таверне!");
}
