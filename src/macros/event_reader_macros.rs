#[macro_export]
macro_rules! read_single_field_variant {
    ($reader:expr, $variant:path) => {
        $reader.read().filter_map(|event| match event {
            $variant(value) => Some(value),
            _ => None,
        })
    };
}
