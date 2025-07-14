use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CardsOrderingSystemSet {
    OriginSetting,
    NewOriginSetTweenFiring,
}

pub struct CardsSystemSetsPlugin;

impl Plugin for CardsSystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ((
                CardsOrderingSystemSet::OriginSetting,
                CardsOrderingSystemSet::NewOriginSetTweenFiring,
            )
                .chain(),),
        );
    }
}
