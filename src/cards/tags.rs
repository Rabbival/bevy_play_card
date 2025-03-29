use crate::prelude::*;

/// A tag added to dragged cards,
/// indicating either that they're being dragged or that they're going back to place after being dragged
#[derive(Debug, Clone, Copy, Component, Default)]
pub enum Dragged {
    /// The card is being actively dragged
    #[default]
    Actively,
    /// The card is no longer being dragged and is currently going back to its origin place
    GoingBackToPlace,
}

/// A tag added to hovered cards, indicating that they're hovered over
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Hovered;

/// A tag added to cards on click, removed on the next click
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Picked;
