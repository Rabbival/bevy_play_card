use bevy_play_card::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_cards_on_a_line).chain())
        .run();
}

fn spawn_cards_on_a_line(
    mut card_line_request_writer: EventWriter<CardLineRequest>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let cards_count = 12;
    let line = commands
        .spawn(CardLineBundle::from_card_line(
            CardLine::default()
                .with_max_cards(cards_count)
                .with_card_origin_gap(100.0)
                .with_picked_cards_capacity(3),
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
                    CardBundle::new(Transform::from_scale(Vec3::splat(0.4))),
                ))
                .id(),
        );
    }
    card_line_request_writer.write(CardLineRequest {
        line,
        request_type: CardLineRequestType::BatchAddToLine { card_entities },
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
