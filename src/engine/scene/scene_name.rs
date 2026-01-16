pub struct SceneName {
    name: String,
}

impl SceneName {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.trim().to_string(),
        }
    }

    pub fn get(&self) -> &str {
        &self.name
    }
}
