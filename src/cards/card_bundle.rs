use crate::prelude::*;

/// Allows for a quick initiation of both card origin and spawn transform, assuming they should be the same
#[derive(Bundle, Debug)]
pub struct CardBundle {
    /// The card itself
    card: Card,
    /// The card's transform component
    transform: Transform,
}

impl CardBundle {
    /// Initiates both Transform component and card origin to the given transform
    pub fn new(transform: Transform) -> Self {
        Self {
            card: Card {
                origin: transform,
                owner_line: None,
            },
            transform,
        }
    }
}
