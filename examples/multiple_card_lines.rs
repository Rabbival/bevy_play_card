use bevy_play_card::prelude::*;
use std::f32::consts::PI;

#[derive(Resource, Default)]
struct CardLineEntities(Vec<Entity>);

#[derive(Event)]
struct SpawnCardPlease;

const MAX_CARDS: usize = 4;

fn main() {
    App::new()
        .init_resource::<CardLineEntities>()
        .add_event::<SpawnCardPlease>()
        .add_plugins((
            DefaultPlugins,
            BevyCardPlugin {
                card_hover_height: 40.0,
                ..default()
            },
        ))
        .add_systems(
            Startup,
            (setup, spawn_card_lines, request_initial_card_spawn).chain(),
        )
        .add_systems(
            Update,
            (listen_to_keyboard_input, listen_to_card_addition_requests).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Text::new(
        "Press Space to raise the card lines\nPress S to spawn another card if possible.",
    ),));
}

fn spawn_card_lines(mut line_entities: ResMut<CardLineEntities>, mut commands: Commands) {
    let distance_from_center = 300.0;
    for (rotation, location) in [
        (0.0, Vec3::NEG_Y * distance_from_center),
        (PI * 0.5, Vec3::X * distance_from_center),
        (PI, Vec3::Y * distance_from_center),
        (PI * 1.5, Vec3::NEG_X * distance_from_center),
    ] {
        line_entities.0.push(
            commands
                .spawn(CardLineBundle::from_card_line(
                    CardLine::default()
                        .with_origin(
                            Transform::from_translation(location.into())
                                .with_rotation(Quat::from_rotation_z(rotation)),
                        )
                        .with_max_cards(MAX_CARDS)
                        .with_card_origin_gap(60.0),
                ))
                .id(),
        );
    }
}

fn request_initial_card_spawn(mut spawn_request_writer: EventWriter<SpawnCardPlease>) {
    spawn_request_writer.write(SpawnCardPlease);
}

fn listen_to_card_addition_requests(
    mut spawn_request_reader: EventReader<SpawnCardPlease>,
    mut card_line_request_writer: EventWriter<CardLineRequest>,
    line_entities: Res<CardLineEntities>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for _ in spawn_request_reader.read() {
        let mut card_addition_requests = vec![];
        for line_entity in &line_entities.0 {
            let card_entity = commands
                .spawn((
                    Sprite {
                        image: asset_server.load("PlaceholderCard.png"),
                        ..default()
                    },
                    CardBundle::new(Transform::from_scale(Vec3::splat(0.25))),
                ))
                .id();
            card_addition_requests.push(CardLineRequest {
                line: *line_entity,
                request_type: CardLineRequestType::AddToLine { card_entity },
            });
        }
        card_line_request_writer.write_batch(card_addition_requests);
    }
}

fn listen_to_keyboard_input(
    mut spawn_request_writer: EventWriter<SpawnCardPlease>,
    mut card_line_request_writer: EventWriter<CardLineRequest>,
    keys: Res<ButtonInput<KeyCode>>,
    cards: Query<(), With<Card>>,
    line_entities: Res<CardLineEntities>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for line_entity in &line_entities.0 {
            card_line_request_writer.write(CardLineRequest {
                line: *line_entity,
                request_type: CardLineRequestType::RaiseLine,
            });
        }
    }
    if keys.just_released(KeyCode::Space) {
        for line_entity in &line_entities.0 {
            card_line_request_writer.write(CardLineRequest {
                line: *line_entity,
                request_type: CardLineRequestType::LowerLine,
            });
        }
    }
    if keys.just_pressed(KeyCode::KeyS) {
        if cards.iter().count() < MAX_CARDS * line_entities.0.len() {
            spawn_request_writer.write(SpawnCardPlease);
        }
    }
}
