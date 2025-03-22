use crate::prelude::*;

#[derive(Bundle, Debug)]
pub struct CardBundle {
    card: Card,
    transform: Transform,
}

impl CardBundle {
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
