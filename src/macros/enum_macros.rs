#[macro_export]
macro_rules! collect_all {
    () => {
        all::<Self>().collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! all_no_field_variant {
    ($iterable:expr, $variant:path) => {
        $iterable
            .into_iter()
            .filter(|event| matches!(event, $variant))
    };
}

#[macro_export]
macro_rules! all_single_field_variant {
    ($iterable:expr, $variant:path) => {
        $iterable.into_iter().filter_map(|event| match event {
            $variant(value) => Some(value),
            _ => None,
        })
    };
}

#[macro_export]
macro_rules! all_two_field_variant {
    ($iterable:expr, $variant:path) => {
        $iterable.into_iter().filter_map(|event| match event {
            $variant(value1, value2) => Some((value1, value2)),
            _ => None,
        })
    };
}
