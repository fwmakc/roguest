use console::{Key, Term};

pub struct InputHandler {
    history: Vec<Key>,
    max_history: usize,
    term: Term,
}

impl InputHandler {
    pub fn new(max_history: usize) -> Self {
        Self {
            history: Vec::with_capacity(max_history),
            max_history,
            term: Term::stdout(),
        }
    }

    /// Считывает одну клавишу и сохраняет её в историю
    pub fn capture(&mut self) -> Option<Key> {
        if let Ok(key) = self.term.read_key() {
            self.history.push(key.clone());
            if self.history.len() > self.max_history {
                self.history.remove(0);
            }
            return Some(key);
        }
        None
    }

    /// Проверяет, совпадает ли конец истории с заданной последовательностью
    pub fn check_sequence(&self, sequence: &[Key]) -> bool {
        if self.history.len() < sequence.len() {
            return false;
        }
        let start = self.history.len() - sequence.len();
        &self.history[start..] == sequence
    }

    /// Очищает историю (например, после успешного ввода комбо)
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
