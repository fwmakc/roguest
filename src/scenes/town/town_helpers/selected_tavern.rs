use crate::{
    engine::{Game, Scene},
    interface::prints,
    scenes::TownScene,
};

pub fn selected_tavern(scene: &mut TownScene, game: &mut Game) {
    scene.deactivate();

    if let Some(ref mut tavern_scene) = game.find_scene("TavernScene") {
        tavern_scene.activate();
    }
}
