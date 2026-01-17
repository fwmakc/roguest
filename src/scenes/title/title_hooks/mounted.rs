use crate::{
    engine::{Game, Scene},
    scenes::{self, TitleScene, title::title_helpers},
};

pub fn mounted(scene: &mut TitleScene, game: &mut Game) {
    title_helpers::print_logo();
    title_helpers::create_player(game);

    scene.deactivate();

    if let Some(ref mut town_scene) = game.find_scene("TownScene") {
        town_scene.activate();
    }
}
