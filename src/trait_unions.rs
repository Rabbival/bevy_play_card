use std::fmt::Debug;

use crate::trait_union;

trait_union!(Sendable, Clone + Send + Sync + 'static + Debug);
