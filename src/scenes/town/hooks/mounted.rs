use crate::{
    engine::{Game, Scene},
    scenes::{TitleScene, title::helpers},
};

pub fn mounted(scene: &mut TitleScene, game: &mut Game) {
    helpers::print_logo();
    helpers::create_player(game);

    scene.deactivate();
}
