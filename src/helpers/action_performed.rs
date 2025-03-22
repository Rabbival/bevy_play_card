#[derive(Debug, Clone, Copy)]
pub struct ActionPerformed(pub bool);

impl ActionPerformed {
    pub fn done(&self) -> bool {
        self.0
    }
}
