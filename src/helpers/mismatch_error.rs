use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MismatchError<'a> {
    pub expected: &'a String,
    pub found: &'a String,
}

impl Display for MismatchError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "expected {} but found {}", self.expected, self.found)
    }
}
