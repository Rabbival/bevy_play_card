use bevy_card::prelude::*;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_a_card).chain())
        .run();
}

fn spawn_a_card(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Sprite {
            image: asset_server.load("PlaceholderCard.png"),
            ..default()
        },
        CardBundle::new(Transform::from_scale(Vec3::splat(0.5))),
    ));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
