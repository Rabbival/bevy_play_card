use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    ListeningPreparations,
    Listening,
    Handling,
    PostHandling,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSystemSet {
    VelocityClamping,
    DummyVariant,
}

pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                (
                    InputSystemSet::ListeningPreparations,
                    InputSystemSet::Listening,
                    InputSystemSet::Handling,
                    InputSystemSet::PostHandling,
                )
                    .chain(),
                PhysicsSystemSet::VelocityClamping.after(InputSystemSet::PostHandling),
            ),
        );
    }
}
