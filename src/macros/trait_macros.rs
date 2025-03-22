#[macro_export]
macro_rules! trait_union {
    ($new_trait_name:ident, $($existing_traits:tt)+) => {
        pub trait $new_trait_name: $($existing_traits)+ {}
        impl<T> $new_trait_name for T where T: $($existing_traits)+ {}
    };
}

#[macro_export]
macro_rules! plugin_for_implementors_of_trait {
    ($plugin_name:ident, $trait_name:ident) => {
        pub struct $plugin_name<T: $trait_name> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<T: $trait_name> Default for $plugin_name<T> {
            fn default() -> Self {
                Self {
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}
