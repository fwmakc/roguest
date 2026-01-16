use crate::{
    engine::{Game, Scene},
    scenes::{TownScene, town::town_helpers::welcome},
};

pub fn activated(scene: &mut TownScene) {
    welcome();
}
