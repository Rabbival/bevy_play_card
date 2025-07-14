use bevy_play_card::cards::card_consts::CardConsts;
use bevy_play_card::prelude::ops::FloatPow;
use bevy_play_card::prelude::*;
use bevy_tween::combinator::{AnimationBuilderExt, TransformTargetStateExt};
use bevy_tween::prelude::{EaseKind, IntoTarget};
use bevy_tween_helpers::prelude::{TweenPriorityToOthersOfType, named_tween};
use std::f32::consts::PI;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_card_line))
        .add_systems(
            Update,
            (
                listen_to_card_addition_requests,
                fix_card_y_to_arc_on_origin_change.in_set(CardsOrderingSystemSet::OriginSetting),
                arc_cards_on_origin_set.in_set(CardsOrderingSystemSet::NewOriginSetTweenFiring),
            )
                .chain(),
        )
        .add_observer(card_hover_animation_override)
        .add_observer(card_pick_animation_override)
        .run();
}

fn card_hover_animation_override(
    trigger: Trigger<OnAdd, Hovered>,
    cards: Query<(&Transform, &Card, &Name)>,
    card_consts: Res<CardConsts>,
    commands: Commands,
) {
    override_card_float_up_animation(
        trigger.target(),
        10 + 1,
        "on-hover-override",
        cards,
        card_consts,
        commands,
    );
}

fn card_pick_animation_override(
    trigger: Trigger<OnAdd, Picked>,
    cards: Query<(&Transform, &Card, &Name)>,
    card_consts: Res<CardConsts>,
    commands: Commands,
) {
    override_card_float_up_animation(
        trigger.target(),
        50 + 1,
        "on-pick-override",
        cards,
        card_consts,
        commands,
    );
}

fn override_card_float_up_animation(
    card_entity: Entity,
    animation_priority: u32,
    animation_name: &str,
    cards: Query<(&Transform, &Card, &Name)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    if let Ok((transform, card, name)) = cards.get(card_entity) {
        let target_translation =
            card.origin.translation + transform.up() * card_consts.card_hover_height + Vec3::Z;
        let animation_target = card_entity.into_target();
        let mut transform_state = animation_target.transform_state(*transform);
        commands
            .spawn((
                Name::new(format!("{} animation parent for {}", animation_name, name)),
                TweenPriorityToOthersOfType(animation_priority),
            ))
            .animation()
            .insert(named_tween(
                Duration::from_secs_f32(card_consts.on_hover_position_tween_duration),
                EaseKind::CubicOut,
                transform_state.translation_to(target_translation),
                format!("{} {} translation tween", name, animation_name),
            ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_card_line(mut commands: Commands) {
    commands.spawn(CardLineBundle::from_card_line(CardLine::default()));
}

fn listen_to_card_addition_requests(
    mut card_line_request_writer: EventWriter<CardLineRequest>,
    card_lines: Query<(&CardLine, Entity)>,
    cards: Query<(), With<Card>>,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if let Ok((card_line, card_line_entity)) = card_lines.single() {
        if keys.just_pressed(KeyCode::KeyS) && cards.iter().count() < card_line.max_cards {
            let card_entity = commands
                .spawn((
                    Sprite {
                        image: asset_server.load("PlaceholderCard.png"),
                        ..default()
                    },
                    CardBundle::new(Transform::from_scale(Vec3::splat(0.5))),
                ))
                .id();
            card_line_request_writer.write(CardLineRequest {
                line: card_line_entity,
                request_type: CardLineRequestType::AddToLine { card_entity },
            });
        }
    }
}

fn fix_card_y_to_arc_on_origin_change(
    changed_card_lines: Query<&CardLine, Changed<CardLine>>,
    mut cards: Query<&mut Card>,
) {
    for card_line in &changed_card_lines {
        for (index, card_entity) in card_line.cards_in_order().iter().enumerate() {
            if let Ok(mut card) = cards.get_mut(*card_entity) {
                let index_delta_from_center =
                    index as f32 - ((card_line.cards_in_order().len() as f32 - 1.0) / 2.0);
                let target_y = index_delta_from_center.squared() * -10.0;
                let resulting_translation = card.origin.translation.with_y(target_y);
                card.origin.translation = resulting_translation;
            }
        }
    }
}

fn arc_cards_on_origin_set(
    cards: Query<(Entity, &Card, &Transform, &Name), Changed<Card>>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    for (card_entity, card, card_transform, card_name) in &cards {
        if *card_transform == card.origin {
            continue;
        }
        rotate_card_on_arc(
            card,
            card_entity,
            card_transform,
            card_name,
            &card_consts,
            &mut commands,
        );
    }
}

fn rotate_card_on_arc(
    card: &Card,
    card_entity: Entity,
    card_transform: &Transform,
    card_name: &Name,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    let target_rotation = -card.origin.translation.x / (PI * 300.0);
    let animation_target = card_entity.into_target();
    let mut transform_state = animation_target.transform_state(*card_transform);
    commands
        .spawn((
            Name::new(format!("Card arcer animation parent for {}", card_name)),
            TweenPriorityToOthersOfType(40),
        ))
        .animation()
        .insert(named_tween(
            Duration::from_secs_f32(card_consts.card_slide_on_origin_set_duration),
            EaseKind::CubicOut,
            transform_state.rotation_to(Quat::from_rotation_z(target_rotation)),
            format!("{} new-origin-set rotation tween", card_name),
        ));
}
