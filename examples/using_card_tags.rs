use bevy_play_card::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_cards).chain())
        .add_systems(Update, print_going_back_to_place_card_names)
        .add_observer(notify_on_hover_start)
        .add_observer(notify_on_picked_cards)
        .run();
}

fn spawn_cards(
    mut card_line_request_writer: EventWriter<CardLineRequest>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let line = commands
        .spawn(CardLineBundle::from_card_line(CardLine::default()))
        .id();
    let mut card_entities = vec![];
    for _ in 0..6 {
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

fn print_going_back_to_place_card_names(
    cards: Query<(&Name, &Dragged), (With<Card>, Changed<Dragged>)>,
) {
    for (card_name, dragged) in &cards {
        if let Dragged::GoingBackToPlace = dragged {
            info!("{} is going back to place", card_name);
        }
    }
}

fn notify_on_hover_start(trigger: Trigger<OnAdd, Hovered>, card_names: Query<&Name, With<Card>>) {
    if let Ok(card_name) = card_names.get(trigger.target()) {
        info!("Hovering {}", card_name);
    }
}

fn notify_on_picked_cards(
    trigger: Trigger<OnAdd, Picked>,
    card_names: Query<(&Name, Option<&Picked>), With<Card>>,
) {
    if let Ok((card_name, _)) = card_names.get(trigger.target()) {
        info!("Picked {}", card_name);
        let picked_cards: Vec<&Name> = card_names
            .iter()
            .filter_map(|(name, picked)| picked.map(|_| name))
            .collect();
        info!("Now the picked cards are {:?}", picked_cards);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
