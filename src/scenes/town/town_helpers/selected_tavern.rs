use crate::{
    engine::{Game, Scene},
    interface::prints,
    scenes::TownScene,
};

pub fn selected_tavern(scene: &mut TownScene, game: &mut Game) {
    scene.deactivate();
    game.activate_scene("TavernScene");
}
