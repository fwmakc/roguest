use crate::{
    engine::{Game, Scene},
    interface::prints,
    scenes::TavernScene,
};

pub fn selected_back_to_town(scene: &mut TavernScene, game: &mut Game) {
    scene.deactivate();
    game.activate_scene("TownScene");
}
