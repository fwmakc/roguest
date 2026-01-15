use crate::{
    engine::{Game, Scene},
    scenes::title::helpers,
};

pub fn create(scene: &mut Scene, game: &mut Game) {
    helpers::print_logo();
    helpers::create_player(game);

    scene.deactivate();
}
