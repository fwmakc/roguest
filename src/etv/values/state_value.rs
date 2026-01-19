pub struct StateValue<T> {
    pub value: T,
}

impl<T> StateValue<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn get(&self) -> T {
        self.value
    }

    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
    }
}
