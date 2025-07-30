use crate::prelude::*;
use bevy_tween::tween_event::TweenEventPlugin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Default)]
pub struct DeclareDraggingDoneForCard {
    pub card_entity: Option<Entity>,
}

#[derive(Debug, Clone, Copy, Event)]
pub struct TogglePickingForCard(pub Entity);

pub struct CardsEventsPlugin;

impl Plugin for CardsEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweenEventPlugin::<DeclareDraggingDoneForCard>::default())
            .add_event::<TogglePickingForCard>();
    }
}
