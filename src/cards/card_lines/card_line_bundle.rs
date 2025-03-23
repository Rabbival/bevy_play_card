use crate::prelude::*;

#[derive(Bundle, Debug)]
pub struct CardLineBundle {
    line: CardLine,
    transform: Transform,
}

impl CardLineBundle {
    pub fn from_transform(transform: Transform) -> Self {
        Self {
            line: CardLine::default().with_origin(transform),
            transform,
        }
    }

    pub fn from_card_line(line: CardLine) -> Self {
        let transform = line.origin.clone();
        Self { line, transform }
    }
}
