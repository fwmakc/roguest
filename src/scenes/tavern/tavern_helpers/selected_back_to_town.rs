use crate::{
    engine::{Game, Scene},
    interface::prints,
    scenes::TavernScene,
};

pub fn selected_back_to_town(scene: &mut TavernScene, game: &mut Game) {
    scene.deactivate();

    if let Some(ref mut town_scene) = game.find_scene("TownScene") {
        town_scene.activate();
    }
}
