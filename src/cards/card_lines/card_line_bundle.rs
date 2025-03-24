use crate::prelude::*;

/// Allows for a quick initiation of both card origin and spawn transform, assuming they should be the same
#[derive(Bundle, Debug)]
pub struct CardLineBundle {
    /// The card-line itself
    line: CardLine,
    /// The card-line's transform component
    transform: Transform,
}

impl CardLineBundle {
    /// Initiates both card-line and Transform component to the given transform
    pub fn from_transform(transform: Transform) -> Self {
        Self {
            line: CardLine::default().with_origin(transform),
            transform,
        }
    }

    /// Initiates from a given card-line, assigning the provided origin value to the Transform component
    pub fn from_card_line(line: CardLine) -> Self {
        let transform = line.origin.clone();
        Self { line, transform }
    }
}
