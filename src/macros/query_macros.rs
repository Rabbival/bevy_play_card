#[macro_export]
macro_rules! get_from_both_queries {
    ($first_query:expr, $second_query:expr, $first_entity:expr, $second_entity:expr) => {{
        $first_query
            .get($first_entity)
            .ok()
            .and_then(|unchangeable| {
                $second_query
                    .get($second_entity)
                    .ok()
                    .map(|physical| (unchangeable, physical))
            })
            .or_else(|| {
                $first_query
                    .get($second_entity)
                    .ok()
                    .and_then(|unchangeable| {
                        $second_query
                            .get($first_entity)
                            .ok()
                            .map(|physical| (unchangeable, physical))
                    })
            })
    }};
}
