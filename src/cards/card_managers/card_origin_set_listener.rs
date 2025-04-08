use crate::cards::card_consts::CardConsts;
use crate::prelude::*;
use bevy_tween::combinator::{AnimationBuilderExt, TransformTargetStateExt, parallel};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::IntoTarget;
use std::time::Duration;

pub struct CardOriginSetListenerPlugin;

impl Plugin for CardOriginSetListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_to_card_origin_changes.in_set(CardsOrderingSystemSet::NewOriginSetTweenFiring),
        );
    }
}

fn listen_to_card_origin_changes(
    cards: Query<(Entity, &Card, &Transform, Option<&Dragged>, &Name), Changed<Card>>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    for (card_entity, card, card_transform, maybe_dragged, card_name) in &cards {
        if let Some(Dragged::Actively) = maybe_dragged {
            continue;
        }
        if *card_transform == card.origin {
            continue;
        }
        let animation_target = card_entity.into_target();
        let mut transform_state = animation_target.transform_state(*card_transform);
        let mut animation_entity = commands.spawn((
            Name::new(format!(
                "Card new origin set animation parent for {}",
                card_name
            )),
            TweenPriorityToOthersOfType(40),
        ));
        let translation_tween = named_tween(
            Duration::from_secs_f32(card_consts.card_slide_on_origin_set_duration),
            EaseKind::CubicOut,
            transform_state.translation_to(card.origin.translation),
            format!("{} new-origin-set translation tween", card_name),
        );
        let scale_tween = named_tween(
            Duration::from_secs_f32(card_consts.card_slide_on_origin_set_duration),
            EaseKind::CubicOut,
            transform_state.scale_to(card.origin.scale),
            format!("{} new-origin-set scale tween", card_name),
        );
        match (
            card_transform.translation != card.origin.translation,
            card_transform.scale != card.origin.scale,
        ) {
            (true, true) => {
                animation_entity
                    .animation()
                    .insert(parallel((translation_tween, scale_tween)));
            }
            (true, false) => {
                animation_entity.animation().insert(translation_tween);
            }
            (false, true) => {
                animation_entity.animation().insert(scale_tween);
            }
            (false, false) => {}
        }
    }
}
