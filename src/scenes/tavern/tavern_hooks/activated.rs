use crate::{
    engine::{Game, Scene},
    interface::prints,
    scenes::TavernScene,
};

pub fn activated(scene: &mut TavernScene) {
    prints::scroll("Таверна «Танцующий пончик»");
    prints::message("Вы заходите в шумную таверну");
}
