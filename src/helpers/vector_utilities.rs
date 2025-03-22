use rand::Rng;

pub fn swap_remove_by_value<T: PartialEq>(
    item_to_remove: &T,
    vec_to_remove_from: &mut Vec<T>,
) -> Option<T> {
    let optional_index_to_remove = item_to_index(item_to_remove, vec_to_remove_from);
    optional_index_to_remove.map(|index_to_remove| vec_to_remove_from.swap_remove(index_to_remove))
}

pub fn remove_by_value<T: PartialEq>(
    item_to_remove: &T,
    vec_to_remove_from: &mut Vec<T>,
) -> Option<T> {
    let optional_index_to_remove = item_to_index(item_to_remove, vec_to_remove_from);
    optional_index_to_remove.map(|index_to_remove| vec_to_remove_from.remove(index_to_remove))
}

pub fn item_to_index<T: PartialEq>(item_to_find: &T, vec_to_find_in: &Vec<T>) -> Option<usize> {
    vec_to_find_in.iter().position(|x| *x == *item_to_find)
}

pub fn random_value<T: Clone>(vec_ref: &Vec<T>) -> T {
    let random_index = random_index(vec_ref);
    vec_ref[random_index].clone()
}

pub fn random_index<T>(vec_ref: &Vec<T>) -> usize {
    rand::rng().random_range(0..vec_ref.len())
}
