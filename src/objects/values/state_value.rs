pub struct StateValue {
    variants: Vec<String>,
    pub value: String,
}

impl StateValue {
    pub fn new<S: Into<String>>(variants: Vec<S>, current: String) -> Self {
        let variants_vec: Vec<String> = variants.into_iter().map(|s| s.into()).collect();
        let current_val: String = current.into();

        if !variants_vec.contains(&current_val) {
            panic!("Начальное значение должно быть в списке вариантов");
        }

        Self {
            variants: variants_vec,
            value: current_val,
        }
    }

    // pub fn new(variants: Vec<String>, value: String) -> Self {
    //     if !variants.contains(&value) {
    //         panic!("Начальное значение должно быть в списке вариантов");
    //     }
    //     Self { variants, value }
    // }

    pub fn get(&self) -> String {
        self.value.clone()
    }

    pub fn set(&mut self, new_val: &str) -> Result<(), String> {
        let new_value = new_val.into();
        if self.variants.contains(&new_value) {
            self.value = new_value;
            Ok(())
        } else {
            Err(format!("Значение '{}' не разрешено", new_value))
        }
    }
}
