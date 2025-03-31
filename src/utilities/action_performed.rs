#[derive(Debug, Clone, Copy)]
pub struct ActionPerformed(pub bool);

impl ActionPerformed {
    pub fn done(&self) -> bool {
        self.0
    }
}

impl std::ops::Not for ActionPerformed {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.0
    }
}
