use crate::prelude::*;

pub struct CardsSpawnerPlugin;

impl Plugin for CardsSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_debug_cards_per_debug_request);
    }
}

fn spawn_debug_cards_per_debug_request(
    mut debug_event_listener: EventReader<DebugEvent>,
    mut card_line_request_writer: EventWriter<CardLineRequest>,
    card_line_entities: Res<CardLineEntities>,
    asset_server: Res<AssetServer>,
    mut card_namer: ResMut<CardNamer>,
    mut commands: Commands,
) {
    for _ in debug_event_listener.read() {
        if let Some(player_card_line) = card_line_entities.player_card_line {
            let card_entity = spawn_debug_card(
                &asset_server,
                Vec3::new(WINDOW_SIZE_IN_PIXELS, 0.0, 0.0),
                card_namer.make_name(),
                &mut commands,
            );
            card_line_request_writer.send(CardLineRequest {
                line: player_card_line,
                request_type: CardLineRequestType::AddToCardLine { card_entity },
            });
        }
        for debug_line in &card_line_entities.debug_card_lines {
            let card_entity = spawn_debug_card(
                &asset_server,
                Vec3::new(0.0, 0.0, 0.0),
                card_namer.make_name(),
                &mut commands,
            );
            card_line_request_writer.send(CardLineRequest {
                line: *debug_line,
                request_type: CardLineRequestType::AddToCardLine { card_entity },
            });
        }
    }
}

fn spawn_debug_card(
    asset_server: &Res<AssetServer>,
    spawn_location: Vec3,
    card_name: Name,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn((
            card_name,
            Sprite {
                image: asset_server.load("sprites/cards/PlaceholderCard.png"),
                ..default()
            },
            CardBundle::new(
                Transform::from_translation(spawn_location.into()).with_scale(Vec3::splat(0.5)),
            ),
        ))
        .id()
}
