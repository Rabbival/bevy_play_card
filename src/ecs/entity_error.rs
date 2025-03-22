use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum EntityError<'a> {
    EntityNotInQuery(&'a str),
    CommandsCouldntGetEntity(&'a str),
}

impl Display for EntityError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EntityNotInQuery(error_message) => {
                write!(f, "Error getting entity, {}", error_message)
            }
            Self::CommandsCouldntGetEntity(entity_type) => {
                write!(f, "Commands couldn't get entity, {}", entity_type)
            }
        }
    }
}
