pub struct SceneActive {
    active: bool,
}

impl SceneActive {
    pub fn new(active: bool) -> Self {
        Self { active }
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
