use crate::cards::card_consts::CardConsts;
use crate::prelude::*;
use bevy_tween::combinator::{AnimationBuilderExt, TransformTargetStateExt, parallel};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::IntoTarget;
use std::time::Duration;

pub struct CardTagInsertionListenerPlugin;

impl Plugin for CardTagInsertionListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_dragged_insertion)
            .add_observer(on_hovered_insertion)
            .add_observer(on_picked_insertion);
    }
}

fn on_dragged_insertion(
    _trigger: Trigger<OnAdd, Dragged>,
    picked_cards: Query<Entity, (With<Card>, With<Picked>)>,
    mut commands: Commands,
) {
    for picked_card_entity in &picked_cards {
        commands.entity(picked_card_entity).remove::<Picked>();
        commands.entity(picked_card_entity).remove::<Hovered>();
    }
}

fn on_hovered_insertion(
    trigger: Trigger<OnAdd, Hovered>,
    cards: Query<(&Transform, &Card, &Name)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    play_card_float_up_animation(
        trigger.target(),
        10,
        "on-hover",
        &cards,
        &card_consts,
        &mut commands,
    );
}

fn on_picked_insertion(
    trigger: Trigger<OnAdd, Picked>,
    cards: Query<(&Transform, &Card, &Name)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    play_card_float_up_animation(
        trigger.target(),
        50,
        "on-picked",
        &cards,
        &card_consts,
        &mut commands,
    );
}

fn play_card_float_up_animation(
    card_to_animate: Entity,
    animation_priority: u32,
    animation_name: &str,
    cards: &Query<(&Transform, &Card, &Name)>,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    if let Ok((transform, card, name)) = cards.get(card_to_animate) {
        let animation_target = card_to_animate.into_target();
        let mut transform_state = animation_target.transform_state(*transform);
        commands
            .spawn((
                Name::new(format!("{} animation parent for {}", animation_name, name)),
                TweenPriorityToOthersOfType(animation_priority),
            ))
            .animation()
            .insert(parallel((
                named_tween(
                    Duration::from_secs_f32(card_consts.on_hover_scale_duration),
                    EaseKind::Linear,
                    transform_state.scale_to(card_consts.on_hover_scale_factor * card.origin.scale),
                    format!("{} {} scaling tween", name, animation_name),
                ),
                named_tween(
                    Duration::from_secs_f32(card_consts.on_hover_position_tween_duration),
                    EaseKind::CubicOut,
                    transform_state.translation_to(
                        card.origin
                            .translation
                            .with_y(card_consts.card_hover_height)
                            + Vec3::Z,
                    ),
                    format!("{} {} translation tween", name, animation_name),
                ),
            )));
    }
}
