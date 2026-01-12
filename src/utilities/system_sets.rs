use crate::prelude::*;
use bevy_tween_helpers::prelude::TweenHelpersSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CardsOrderingSystemSet {
    OriginSetting,
    NewOriginSetTweenFiring,
    CardPickingRequestListening,
    CardAnimationRequesting,
    CardAnimation,
}

pub struct CardsSystemSetsPlugin;

impl Plugin for CardsSystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ((
                CardsOrderingSystemSet::OriginSetting,
                CardsOrderingSystemSet::NewOriginSetTweenFiring,
                (
                    CardsOrderingSystemSet::CardAnimationRequesting,
                    CardsOrderingSystemSet::CardAnimation,
                )
                    .chain()
                    .after(CardsOrderingSystemSet::CardPickingRequestListening),
            )
                .chain()
                .before(TweenHelpersSystemSet::PreTargetRemoval),),
        );
    }
}
