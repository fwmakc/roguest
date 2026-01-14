use crate::eco::components::RangeConfig;
use crate::eco::entities::creature::{Creature, CreatureConfig};

pub fn player(player_name: String) -> Creature {
    let new_player = Creature::new(CreatureConfig {
        name: player_name,
        level: RangeConfig {
            value: 1,
            min: 1,
            max: 100,
        },
        attack: RangeConfig {
            value: 0,
            min: 10,
            max: 20,
            ..RangeConfig::default()
        },
        ..Default::default()
    });

    return new_player;
}
