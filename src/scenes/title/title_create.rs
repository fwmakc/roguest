use crate::{
    engine::{Game, Scene},
    scenes::title::helpers,
};

#[allow(unused)]
pub fn title_create(scene: &mut Scene, game: &mut Game) {
    helpers::print_logo();
    helpers::create_player(game);

    scene.deactivate();
}
