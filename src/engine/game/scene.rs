pub struct Scene {
    active: bool,
    name: String,
}

impl Scene {
    pub fn new(name: &str, active: bool) -> Self {
        Self {
            name: name.trim().to_string(),
            active,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
