macro_rules! define_namer {
    ($namer:ident, $type:ty) => {
        #[derive(Resource, Debug, Default)]
        pub struct $namer {
            named_entities: std::collections::HashMap<$type, Vec<u32>>,
        }

        pub struct $namerPlugin;

        impl bevy::app::Plugin for $namerPlugin {
            fn build(&self, app: &mut bevy::app::App) {
                app.init_resource::<$namer>();
            }
        }

        impl $namer {
            pub fn make_name(&mut self, entity_type: $type) -> String {
                let mut number_string = String::new();
                let entity_count = self.named_entities.entry(entity_type).or_insert(Vec::new());
                match entity_count.last_mut() {
                    None => {
                        entity_count.push(0);
                    }
                    Some(last_count) => {
                        *last_count += 1;
                        if *last_count == u32::MAX {
                            entity_count.push(0);
                        }
                    }
                }
                for count in entity_count {
                    number_string += &count.to_string();
                }
                format!("{:?} {}", entity_type, number_string)
            }
        }
    };
}
