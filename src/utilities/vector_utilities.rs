pub fn remove_by_value<T: PartialEq>(
    item_to_remove: &T,
    vec_to_remove_from: &mut Vec<T>,
) -> Option<T> {
    let optional_index_to_remove = item_to_index(item_to_remove, vec_to_remove_from);
    optional_index_to_remove.map(|index_to_remove| vec_to_remove_from.remove(index_to_remove))
}

pub fn item_to_index<T: PartialEq>(item_to_find: &T, vec_to_find_in: &[T]) -> Option<usize> {
    vec_to_find_in.iter().position(|x| *x == *item_to_find)
}
