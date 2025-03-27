use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CardsOrderingSystemSet {
    OriginSetting,
    SlideToNewOriginTweenFIring,
}

pub struct CardsSystemSetsPlugin;

impl Plugin for CardsSystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ((
                CardsOrderingSystemSet::OriginSetting,
                CardsOrderingSystemSet::SlideToNewOriginTweenFIring,
            )
                .chain(),),
        );
    }
}
