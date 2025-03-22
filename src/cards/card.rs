use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Card {
    pub origin: Transform,
    pub owner_line: Option<Entity>,
}
