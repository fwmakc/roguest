use crate::etv::{
    types::{Creature, CreatureConfig},
    values::RangeConfig,
};

pub fn player(player_name: String) -> Creature {
    Creature::new(CreatureConfig {
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
    })
}
