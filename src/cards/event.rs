use bevy_tween::tween_event::TweenEventPlugin;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Default)]
pub struct DeclareDraggingDoneForCard {
    pub card_entity: Option<Entity>,
}

pub struct CardsEventsPlugin;

impl Plugin for CardsEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweenEventPlugin::<DeclareDraggingDoneForCard>::default());
    }
}
