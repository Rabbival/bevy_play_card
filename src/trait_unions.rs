use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use strum::IntoEnumIterator;

use crate::{prelude::PathableTail, trait_union};

trait_union!(
    Numeric,
    Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self> + Sendable + Default + Copy
);

trait_union!(Sendable, Clone + Send + Sync + 'static + Debug);

trait_union!(
    PathableSendableTail,
    PathableTail + Debug + Sendable + PartialEq + Eq + Hash + PartialOrd + IntoEnumIterator + Copy
);
