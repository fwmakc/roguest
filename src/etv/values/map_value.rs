use std::collections::HashMap;

use rand::distr::Map;

pub struct MapValue<T> {
    pub variants: HashMap<String, T>,
}

impl<T> MapValue<T>
where
    T: Clone,
{
    pub fn new(variants: HashMap<String, T>) -> Self {
        Self { variants }
    }

    pub fn get(&self, value: &str) -> Option<T> {
        self.variants.get(value).cloned()
    }

    pub fn add(&mut self, key: String, value: T) {
        self.variants.insert(key, value);
    }
}
