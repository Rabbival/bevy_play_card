use crate::prelude::*;

#[derive(Component, Debug)]
pub struct CardLine {
    pub cards_in_order: Vec<Entity>,
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
}

impl Default for CardLine {
    fn default() -> Self {
        Self {
            cards_in_order: vec![],
            origin: Transform::default(),
            max_cards: CARD_LINE_MAX_CARDS,
            raised_card_line_delta: RAISED_CARD_LINE_DELTA,
            slide_duration: CARD_LINE_SLIDE_DURATION,
            card_origin_gap: CARD_ORIGIN_GAP,
        }
    }
}
