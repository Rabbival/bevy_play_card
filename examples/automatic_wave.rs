use bevy_play_card::prelude::*;
use std::time::Duration;

const CARDS_COUNT: usize = 10;

#[derive(Resource, Default)]
struct NextPickIndex(usize);

#[derive(Resource, Debug)]
struct TimerWrapper(Timer);

fn main() {
    App::new()
        .init_resource::<NextPickIndex>()
        .insert_resource(TimerWrapper(Timer::new(
            Duration::from_secs_f32(0.12),
            TimerMode::Repeating,
        )))
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_cards_on_a_line).chain())
        .add_systems(Update, pick_cards)
        .run();
}

fn pick_cards(
    mut requester: MessageWriter<TogglePickingForCard>,
    card_lines: Query<&CardLine>,
    mut next: ResMut<NextPickIndex>,
    mut timer: ResMut<TimerWrapper>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.is_finished() {
        return;
    }
    if let Ok(card_line) = card_lines.single()
        && let Some(card_entity) = card_line.cards_in_order().get(next.0)
    {
        requester.write(TogglePickingForCard(*card_entity));
        next.0 = (next.0 + 1) % CARDS_COUNT;
    }
}

fn spawn_cards_on_a_line(
    mut card_line_request_writer: MessageWriter<CardLineRequest>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let line_entity = commands
        .spawn(CardLineBundle::from_card_line(
            CardLine::default()
                .with_max_cards(CARDS_COUNT)
                .with_card_origin_gap(90.0)
                .with_picked_cards_capacity(2)
                .with_card_picking_policy(CardPickingPolicy::RemoveLeastRecentlyPicked),
        ))
        .id();
    let mut card_entities = vec![];
    for _ in 0..CARDS_COUNT {
        card_entities.push(
            commands
                .spawn((
                    Sprite {
                        image: asset_server.load("PlaceholderCard.png"),
                        ..default()
                    },
                    CardBundle::new(Transform::from_scale(Vec3::splat(0.25))),
                ))
                .id(),
        );
    }
    card_line_request_writer.write(CardLineRequest {
        entity: line_entity,
        request_type: CardLineRequestType::BatchAddToLine { card_entities },
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
