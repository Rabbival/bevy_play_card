use bevy_play_card::prelude::*;

#[derive(Component)]
struct CardDestroyer;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_card_line, spawn_card_destroyer))
        .add_systems(Update, listen_to_card_addition_requests)
        .add_observer(listen_to_card_drops)
        .add_observer(listen_to_card_destroyer_clicks)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Text::new(
        "Drag cards to the X to destroy them\n\
        Press S to spawn another card if possible.\n\
        You can also destroy cards by picking them and clicking the X",
    ),));
}

fn spawn_card_line(mut commands: Commands) {
    commands.spawn(CardLineBundle::from_card_line(
        CardLine::default().with_origin(Transform::from_translation(Vec3::new(0.0, -240.0, 1.0))),
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
    card_destroyer: Query<(), With<CardDestroyer>>,
    cards: Query<Entity, With<Card>>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if card_destroyer.get(trigger.target).is_ok() {
        if let Ok(card_entity) = cards.get(trigger.dropped) {
            commands.entity(card_entity).despawn();
        }
    }
}

fn listen_to_card_destroyer_clicks(
    mut trigger: Trigger<Pointer<Click>>,
    card_destroyer: Query<(), With<CardDestroyer>>,
    picked_cards: Query<Entity, (With<Card>, With<Picked>)>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if card_destroyer.get(trigger.target).is_ok() {
        for card_entity in &picked_cards {
            commands.entity(card_entity).despawn();
        }
    }
}
