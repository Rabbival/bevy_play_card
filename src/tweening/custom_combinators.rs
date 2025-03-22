use std::time::Duration;

use bevy_time_runner::TimeSpan;
use combinator::AnimationCommands;

use crate::prelude::*;

pub fn named_tween<I, T>(
    duration: Duration,
    interpolation: I,
    tween: T,
    name: String,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    I: Bundle,
    T: Bundle,
{
    move |a, pos| {
        let start = *pos;
        let end = start + duration;
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            interpolation,
            tween,
            Name::new(name),
        ));
        *pos = end;
    }
}
