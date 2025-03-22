use crate::prelude::*;

#[derive(Bundle, Debug)]
pub struct CardLineBundle {
    line: CardLine,
    transform: Transform,
    visibility: Visibility,
}

impl CardLineBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            line: CardLine {
                origin: transform,
                ..default()
            },
            transform,
            visibility: Visibility::default(),
        }
    }
}
