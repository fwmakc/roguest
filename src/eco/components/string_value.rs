pub struct StringValue {
    pub value: String,
}

impl StringValue {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn get(&self) -> String {
        return self.value.clone();
    }
}
