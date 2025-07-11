use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CardsOrderingSystemSet {
    OriginSetting,
    NewOriginSetTweenFiring,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TweenHelpersSystemSet {
    PreTargetRemoval,
    TargetRemoval,
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
        )
        .configure_sets(
            Update,
            ((
                TweenHelpersSystemSet::PreTargetRemoval,
                TweenHelpersSystemSet::TargetRemoval,
            )
                .chain(),),
        );
    }
}
