use crate::{
    engine::{Game, Scene},
    scenes::{self, TitleScene, title::helpers},
};

pub fn mounted(scene: &mut TitleScene, game: &mut Game) {
    helpers::print_logo();
    helpers::create_player(game);

    // if let Some(ref mut town_scene) = game.find_scene("TownScene") {
    //     // town_scene.activate();
    //     town_scene.activate();
    // };

    scene.deactivate();

    let mut town_scene = scenes::TownScene::new();
    town_scene.activate();

    game.add_scene(town_scene);
}
