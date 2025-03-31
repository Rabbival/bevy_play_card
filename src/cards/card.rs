use crate::prelude::*;

/// The basic component. Can be attached to anything, if you want it to have an origin to snap back to and possibly attach it to a line.
#[derive(Component, Debug)]
#[require(Pickable{ should_block_lower: false, is_hoverable: true})]
pub struct Card {
    /// Keeps track of the card's original transform, so that it can be snapped back to on demand
    pub origin: Transform,
    /// The card's owner card-line entity. If you attach cards manually you should set this field appropriately
    pub owner_line: Option<Entity>,
}
