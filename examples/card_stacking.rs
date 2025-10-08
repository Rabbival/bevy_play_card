use bevy_play_card::prelude::*;

const CARD_STACK_GAP: f32 = 40.0;
const CARD_SCALE: f32 = 0.5;

#[derive(Component, Default)]
struct StackedCardsCount(usize);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_cards_on_a_line).chain())
        .add_observer(listen_to_card_drops)
        .run();
}

fn spawn_cards_on_a_line(
    mut card_line_request_writer: MessageWriter<CardLineRequest>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let cards_count = 6;
    let line_entity = commands
        .spawn(CardLineBundle::from_card_line(
            CardLine::default()
                .with_max_cards(cards_count)
                .with_auto_sort(false),
        ))
        .id();
    let mut card_entities = vec![];
    for _ in 0..cards_count {
        card_entities.push(
            commands
                .spawn((
                    Sprite {
                        image: asset_server.load("PlaceholderCard.png"),
                        ..default()
                    },
                    CardBundle::new(Transform::from_scale(Vec3::splat(CARD_SCALE))),
                    StackedCardsCount::default(),
                ))
                .id(),
        );
    }
    card_line_request_writer.write(CardLineRequest {
        entity: line_entity,
        request_type: CardLineRequestType::BatchAddToLine { card_entities },
    });
    card_line_request_writer.write(CardLineRequest {
        entity: line_entity,
        request_type: CardLineRequestType::Sort,
    });
}

fn listen_to_card_drops(
    mut trigger: On<Pointer<DragDrop>>,
    mut line_request_writer: MessageWriter<CardLineRequest>,
    mut dropped_query: Query<(Entity, &mut Transform, &mut Card), With<Dragged>>,
    mut dropped_onto_query: Query<
        (Entity, &Transform, &mut StackedCardsCount),
        (With<Card>, Without<Dragged>),
    >,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if let Ok((card_dropped_onto_entity, transform_dropped_onto, mut stack_count)) =
        dropped_onto_query.get_mut(trigger.entity)
        && let Ok((dropped_card_entity, mut dropped_card_transform, mut dropped_card)) =
            dropped_query.get_mut(trigger.dropped)
        && let Some(line_entity) = dropped_card.owner_line
    {
        commands.entity(dropped_card_entity).remove::<Pickable>();
        commands
            .entity(card_dropped_onto_entity)
            .add_child(dropped_card_entity);
        stack_count.0 += 1;
        fix_child_card_origin_and_transform(
            transform_dropped_onto,
            &mut dropped_card_transform,
            &mut dropped_card,
            stack_count.0,
        );
        line_request_writer.write(CardLineRequest {
            entity: line_entity,
            request_type: CardLineRequestType::RemoveFromLine {
                card_entity: dropped_card_entity,
            },
        });
        line_request_writer.write(CardLineRequest {
            entity: line_entity,
            request_type: CardLineRequestType::Sort,
        });
    }
}

fn fix_child_card_origin_and_transform(
    transform_dropped_onto: &Transform,
    dropped_card_transform: &mut Transform,
    dropped_card: &mut Card,
    stack_count: usize,
) {
    dropped_card_transform.translation -= transform_dropped_onto.translation;
    dropped_card_transform.scale = Vec3::splat(1.0 / CARD_SCALE);

    let offset = (Vec3::Y * stack_count as f32 + Vec3::NEG_Z) * CARD_STACK_GAP;
    dropped_card.origin.translation = offset;
    dropped_card.origin.scale = Vec3::ONE;
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
