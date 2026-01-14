use crate::eco::components::RangeConfig;
use crate::eco::entities::creature::{Creature, CreatureConfig};
use crate::engine::math;

pub fn goblin() -> Creature {
    return Creature::new(CreatureConfig {
        name: "Гоблин".to_string(),
        level: RangeConfig {
            value: math::random(1, 3),
            ..RangeConfig::default()
        },
        gold: RangeConfig {
            value: math::random(1, 10),
            ..RangeConfig::default()
        },
        hp: RangeConfig {
            value: math::random(10, 30),
            ..RangeConfig::default()
        },
        attack: RangeConfig {
            value: 0,
            min: 5,
            max: 15,
            ..RangeConfig::default()
        },
    });
}
