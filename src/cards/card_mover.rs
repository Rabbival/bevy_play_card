use crate::prelude::*;
use bevy_tween::combinator::{AnimationBuilderExt, TransformTargetStateExt};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::IntoTarget;
use std::time::Duration;

pub struct CardMoverPlugin;

impl Plugin for CardMoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, slide_all_cards_to_their_origins);
    }
}

fn slide_all_cards_to_their_origins(
    cards: Query<(Entity, &Card, &Transform, Option<&Dragged>, &Name), Changed<Card>>,
    mut commands: Commands,
) {
    for (card_entity, card, card_transform, maybe_dragged, card_name) in &cards {
        if let Some(Dragged::Actively) = maybe_dragged {
            return;
        }
        let animation_target = card_entity.into_target();
        let mut transform_state = animation_target.transform_state(*card_transform);
        commands
            .spawn((
                Name::new(format!(
                    "Card slide to new origin animation parent for {}",
                    card_name
                )),
                TweenPriorityToOthersOfType(3),
            ))
            .animation()
            .insert(named_tween(
                Duration::from_secs_f32(CARD_SLIDE_ON_ORIGIN_SET_DURATION),
                EaseKind::CubicOut,
                transform_state.translation_to(card.origin.translation),
                format!("{} card-slide-to-new-origin (translation) tween", card_name),
            ));
    }
}
