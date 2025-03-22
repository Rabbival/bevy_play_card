#[macro_export]
macro_rules! read_no_field_variant {
    ($reader:expr, $variant:path) => {
        $reader.read().filter(|event| matches!(event, $variant))
    };
}

#[macro_export]
macro_rules! read_single_field_variant {
    ($reader:expr, $variant:path) => {
        $reader.read().filter_map(|event| match event {
            $variant(value) => Some(value),
            _ => None,
        })
    };
}

#[macro_export]
macro_rules! read_two_field_variant {
    ($reader:expr, $variant:path) => {
        $reader.read().filter_map(|event| match event {
            $variant(value1, value2) => Some((value1, value2)),
            _ => None,
        })
    };
}
