use bevy_play_card::prelude::*;
use bevy_tween::combinator::{
    AnimationBuilderExt, TransformTargetStateExt, event, parallel, sequence,
};
use bevy_tween::interpolate::sprite_color_to;
use bevy_tween::prelude::{EaseKind, IntoTarget};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Component)]
struct CardDestroyer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Default)]
pub struct DespawnRequest {
    pub entity_to_despawn: Option<Entity>,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_card_line, spawn_card_destroyer))
        .add_systems(Update, listen_to_card_addition_requests)
        .add_observer(listen_to_card_drops)
        .add_observer(listen_to_card_destroyers_clicks)
        .add_observer(
            |trigger: Trigger<TweenEvent<DespawnRequest>>, mut commands: Commands| {
                if let Some(entity) = trigger.data.entity_to_despawn {
                    if let Ok(mut entity_commands) = commands.get_entity(entity) {
                        entity_commands.despawn();
                    }
                }
            },
        )
        .add_plugins(TweenEventPlugin::<DespawnRequest>::default())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Text::new(
        "Drag cards to the Xs to discard them\n\
        Press S to spawn another card if possible.\n\
        You can also destroy cards by picking them and clicking the X",
    ),));
}

fn spawn_card_line(mut commands: Commands) {
    commands.spawn(CardLineBundle::from_card_line(
        CardLine::default().with_origin(Transform::from_translation(Vec3::new(0.0, -220.0, 1.0))),
    ));
}

fn spawn_card_destroyer(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Sprite {
            image: asset_server.load("CardDestructionPitPlaceholder.png"),
            ..default()
        },
        CardDestroyer,
        Pickable::default(),
    ));
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

fn listen_to_card_drops(
    mut trigger: Trigger<Pointer<DragDrop>>,
    card_destroyers: Query<&CardDestroyer>,
    cards: Query<(Entity, &GlobalTransform, &Sprite, &Name), With<Card>>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if card_destroyers.contains(trigger.target) {
        if let Ok((card_entity, card_transform, card_sprite, card_name)) =
            cards.get(trigger.dropped)
        {
            play_despawn_animation_and_despawn(
                card_entity,
                card_transform,
                card_sprite,
                card_name,
                &mut commands,
            );
        }
    }
}

fn listen_to_card_destroyers_clicks(
    mut trigger: Trigger<Pointer<Click>>,
    card_destroyers: Query<&CardDestroyer>,
    picked_cards: Query<(Entity, &GlobalTransform, &Sprite, &Name), (With<Card>, With<Picked>)>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if card_destroyers.contains(trigger.target) {
        for (card_entity, card_transform, card_sprite, card_name) in &picked_cards {
            play_despawn_animation_and_despawn(
                card_entity,
                card_transform,
                card_sprite,
                card_name,
                &mut commands,
            );
        }
    }
}

fn play_despawn_animation_and_despawn(
    card_entity: Entity,
    card_transform: &GlobalTransform,
    card_sprite: &Sprite,
    card_name: &Name,
    commands: &mut Commands,
) {
    if let Ok(mut card_commands) = commands.get_entity(card_entity) {
        card_commands.remove::<Card>(); //it should no longer be considered a card
    }

    let animation_duration = Duration::from_secs_f32(0.6);
    let animation_target = card_entity.into_target();
    let mut transform_state = animation_target.transform_state(card_transform.compute_transform());
    let mut color_state = animation_target.state(card_sprite.color);

    commands
        .spawn((
            Name::new(format!("Card despawn animation parent for {}", card_name)),
            TweenPriorityToOthersOfType(40),
        ))
        .animation()
        .insert(sequence((
            parallel((
                named_tween(
                    animation_duration,
                    EaseKind::Linear,
                    transform_state.translation_by(Vec3::Y * 100.0),
                    format!("{} card despawn translation tween", card_name),
                ),
                named_tween(
                    animation_duration,
                    EaseKind::Linear,
                    color_state.with(sprite_color_to(Color::NONE)),
                    format!("{} card despawn color tween", card_name),
                ),
            )),
            event(DespawnRequest {
                entity_to_despawn: Some(card_entity),
            }),
        )));
}
