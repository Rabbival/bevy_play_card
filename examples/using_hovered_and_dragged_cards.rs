use bevy_play_card::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_a_card).chain())
        .add_systems(Update, print_going_back_to_place_card_names)
        .run();
}

fn spawn_a_card(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn((
            Sprite {
                image: asset_server.load("PlaceholderCard.png"),
                ..default()
            },
            CardBundle::new(Transform::from_scale(Vec3::splat(0.5))),
        ))
        .observe(notify_on_hover_start);
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
