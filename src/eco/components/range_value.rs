use num_traits::{Bounded, Num};
use std::fmt::Display;

pub struct RangeConfig<T> {
    pub value: T,
    pub min: T,
    pub max: T,
}

impl<T> Default for RangeConfig<T>
where
    T: Num + Bounded + Copy + Display,
{
    fn default() -> Self {
        Self {
            value: T::zero(),
            min: T::zero(),
            max: T::max_value(),
        }
    }
}

pub struct RangeValue<T> {
    pub value: T,
    pub min: T,
    pub max: T,
    pub default: T,
}

impl<T> RangeValue<T>
where
    T: Num + Bounded + PartialOrd + Copy + Display,
{
    pub fn new(config: RangeConfig<T>) -> Self {
        let mut instance = Self {
            value: config.value,
            min: config.min,
            max: config.max,
            default: config.value,
        };

        instance.clamp();

        return instance;
    }

    pub fn get(&self) -> T {
        return self.value;
    }

    pub fn max(&self) -> T {
        return self.max;
    }

    pub fn min(&self) -> T {
        return self.min;
    }

    pub fn add(&mut self, value: T) {
        let max_value: T = T::max_value();
        let max_attempt_value = max_value - value;

        if self.value < max_attempt_value {
            self.value = self.value + value;
        } else {
            self.value = max_value;
        }

        self.clamp();
    }

    pub fn sub(&mut self, value: T) {
        let min_value = T::min_value();
        let min_attempt_value = min_value + value;

        if self.value > min_attempt_value {
            self.value = self.value - value;
        } else {
            self.value = min_value;
        }

        self.clamp();
    }

    pub fn clamp(&mut self) {
        if self.value > self.max {
            self.value = self.max;
        }
        if self.value < self.min {
            self.value = self.min;
        }
    }

    pub fn reset(&mut self) {
        self.value = self.default;
    }

    pub fn set_max(&mut self, max: T) {
        self.max = max;
        self.clamp();
    }

    pub fn set_min(&mut self, min: T) {
        self.min = min;
        self.clamp();
    }

    pub fn is_max(&self) -> bool {
        return self.value >= self.max;
    }

    pub fn is_min(&self) -> bool {
        return self.value <= self.min;
    }
}
