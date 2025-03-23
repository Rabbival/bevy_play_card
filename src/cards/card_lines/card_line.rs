use crate::prelude::*;
use crate::utilities::action_performed::ActionPerformed;
use crate::utilities::vector_utilities::remove_by_value;

#[derive(Component, Debug)]
#[require(Visibility)]
pub struct CardLine {
    cards_in_order: Vec<Entity>,
    pub origin: Transform,
    pub max_cards: usize,
    pub raised_card_line_delta: f32,
    pub slide_duration: f32,
    pub card_origin_gap: f32,
}

impl CardLine {
    pub fn insert_if_theres_space(&mut self, card_entity: Entity) -> ActionPerformed {
        let there_was_space = self.cards_in_order.len() < self.max_cards;
        if there_was_space {
            self.cards_in_order.push(card_entity);
        }
        ActionPerformed(there_was_space)
    }

    pub fn remove_card_if_found(&mut self, card_entity: Entity) -> ActionPerformed {
        let removed = remove_by_value(&card_entity, &mut self.cards_in_order).is_some();
        ActionPerformed(removed)
    }

    pub fn heavy_swap(&mut self, old_index: usize, new_index: usize) -> ActionPerformed {
        let performable_and_meaningful = old_index != new_index
            && old_index < self.cards_in_order.len()
            && new_index < self.cards_in_order.len();
        if performable_and_meaningful {
            let value = self.cards_in_order.remove(old_index);
            self.cards_in_order.insert(new_index, value);
        }
        ActionPerformed(performable_and_meaningful)
    }

    pub fn cards_in_order(&self) -> &Vec<Entity> {
        &self.cards_in_order
    }
}

//Builders
impl CardLine {
    pub fn with_origin(mut self, origin: Transform) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_max_cards(mut self, max_cards: usize) -> Self {
        self.max_cards = max_cards;
        self
    }

    pub fn with_raised_card_line_delta(mut self, delta: f32) -> Self {
        self.raised_card_line_delta = delta;
        self
    }

    pub fn with_slide_duration(mut self, duration: f32) -> Self {
        self.slide_duration = duration;
        self
    }

    pub fn with_card_origin_gap(mut self, gap: f32) -> Self {
        self.card_origin_gap = gap;
        self
    }
}

impl Default for CardLine {
    fn default() -> Self {
        Self {
            cards_in_order: vec![],
            origin: Transform::default(),
            max_cards: 6,
            raised_card_line_delta: 100.0,
            slide_duration: 0.3,
            card_origin_gap: 140.0,
        }
    }
}
