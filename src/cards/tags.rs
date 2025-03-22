use crate::prelude::*;

#[derive(Debug, Clone, Copy, Component, Default)]
pub enum Dragged {
    #[default]
    Actively,
    GoingBackToPlace,
}
