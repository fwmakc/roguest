use crate::eco::components::StringValue;
use crate::eco::components::{RangeConfig, RangeValue};
use crate::engine::math;

pub struct CreatureConfig {
    pub name: String,
    pub level: RangeConfig<u8>,
    pub gold: RangeConfig<u16>,
    pub hp: RangeConfig<u16>,
    pub attack: RangeConfig<u16>,
}

impl Default for CreatureConfig {
    fn default() -> Self {
        Self {
            name: "NPC".to_string(),
            level: RangeConfig {
                value: 1,
                min: 1,
                max: 5,
            },
            gold: RangeConfig {
                value: 0,
                ..RangeConfig::default()
            },
            hp: RangeConfig {
                value: 100,
                max: 100,
                ..RangeConfig::default()
            },
            attack: RangeConfig {
                value: 10,
                ..RangeConfig::default()
            },
        }
    }
}

pub struct Creature {
    pub name: StringValue,
    pub level: RangeValue<u8>,
    pub gold: RangeValue<u16>,
    pub hp: RangeValue<u16>,
    pub attack: RangeValue<u16>,
}

impl Creature {
    pub fn new(config: CreatureConfig) -> Self {
        Self {
            name: StringValue::new(config.name),
            level: RangeValue::new(config.level),
            gold: RangeValue::new(config.gold),
            hp: RangeValue::new(config.hp),
            attack: RangeValue::new(config.attack),
        }
    }

    pub fn calculate_attack(&self) -> u16 {
        let attack = math::random(self.attack.min, self.attack.max);
        let koeff: f32 = 1.0 + self.level.get() as f32 / self.level.max() as f32;
        let calculated_attack: u16 = (attack as f32 * koeff) as u16;

        return calculated_attack;
    }
}
