use crate::prelude::*;

pub fn resource_changed_not_added<T>(res: Res<T>) -> bool
where
    T: Resource,
{
    res.is_changed() && !res.is_added()
}
