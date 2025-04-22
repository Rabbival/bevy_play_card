use tracing::info;
use crate::prelude::*;
use crate::utilities::action_performed::ActionPerformed;
use crate::utilities::calculation_helpers::projection_directed_distance;
use crate::utilities::system_sets::CardsOrderingSystemSet;

pub struct CardLinesContentManagerPlugin;

impl Plugin for CardLinesContentManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_to_card_removal_requests,
                listen_to_card_addition_requests,
                listen_to_dragged_card_movements,
                set_card_origins_on_line_change,
            )
                .chain()
                .in_set(CardsOrderingSystemSet::OriginSetting),
        )
        .add_observer(remove_cards_from_line_on_card_despawn);
    }
}

fn remove_cards_from_line_on_card_despawn(
    trigger: Trigger<OnRemove, Card>,
    mut line_request_writer: EventWriter<CardLineRequest>,
    cards: Query<&Card>,
) {
    if let Ok(card) = cards.get(trigger.target()) {
        if let Some(owner_line_entity) = card.owner_line {
            line_request_writer.write(CardLineRequest {
                line: owner_line_entity,
                request_type: CardLineRequestType::RemoveFromLine {
                    card_entity: trigger.target(),
                },
            });
        }
    }
}

fn listen_to_card_removal_requests(
    mut card_line_request_reader: EventReader<CardLineRequest>,
    mut card_lines: Query<&mut CardLine>,
    mut cards: Query<(&mut Card, &Name)>,
    debug_logs_enabled: Res<CardsPluginShouldPrintLogs>,
    mut commands: Commands,
) {
    for request in card_line_request_reader.read() {
        match &request.request_type {
            CardLineRequestType::RemoveFromLine { card_entity } => {
                if let Ok(mut card_line) = card_lines.get_mut(request.line) {
                    remove_card_from_line_if_found(
                        *card_entity,
                        &mut card_line,
                        &mut cards,
                        debug_logs_enabled.0,
                        &mut commands,
                    );
                }
            }
            CardLineRequestType::BatchRemoveFromLine { card_entities } => {
                if let Ok(mut card_line) = card_lines.get_mut(request.line) {
                    for card_entity in card_entities.iter() {
                        remove_card_from_line_if_found(
                            *card_entity,
                            &mut card_line,
                            &mut cards,
                            debug_logs_enabled.0,
                            &mut commands,
                        );
                    }
                }
            }
            CardLineRequestType::RemoveAllCardsFromLine => {
                if let Ok(mut card_line) = card_lines.get_mut(request.line) {
                    let removed_cards = card_line.remove_all_cards();
                    let mut removed_cards_names = vec![];
                    for &card_entity in removed_cards.iter() {
                        if let Ok(mut card_entity_to_remove_commands) =
                            commands.get_entity(card_entity)
                        {
                            card_entity_to_remove_commands.remove_parent_in_place();
                        }
                        if let Ok((mut card, card_name)) = cards.get_mut(card_entity) {
                            card.owner_line = None;
                            removed_cards_names.push(card_name.clone());
                        }
                    }
                    if debug_logs_enabled.0 {
                        info!(
                            "Removed all cards from a line. Cards removed: {:?}",
                            removed_cards_names
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

fn remove_card_from_line_if_found(
    card_entity_to_remove: Entity,
    card_line: &mut CardLine,
    cards: &mut Query<(&mut Card, &Name)>,
    should_log: bool,
    commands: &mut Commands,
) -> ActionPerformed {
    let mut card_name_if_removed = None;
    let card_removed = card_line.remove_card_if_found(card_entity_to_remove);
    if card_removed.done() {
        if let Ok(mut card_entity_to_remove_commands) = commands.get_entity(card_entity_to_remove) {
            card_entity_to_remove_commands.remove_parent_in_place();
        }
        if let Ok((mut card, card_name)) = cards.get_mut(card_entity_to_remove) {
            card.owner_line = None;
            card_name_if_removed = Some(card_name.clone());
        }
    }
    if should_log {
        if let Some(card_name) = card_name_if_removed {
            let mut names_in_order = vec![];
            for card_entity in card_line.cards_in_order() {
                if let Ok((_, name)) = cards.get(*card_entity) {
                    names_in_order.push(name);
                }
            }
            info!(
                "{} was removed from a card line, so it's now: {:?}",
                card_name, names_in_order
            );
        }
    }
    card_removed
}

fn listen_to_card_addition_requests(
    mut card_line_request_reader: EventReader<CardLineRequest>,
    mut card_lines: Query<&mut CardLine>,
    mut cards: Query<(&mut Card, &Name)>,
    debug_logs_enabled: Res<CardsPluginShouldPrintLogs>,
    mut commands: Commands,
) {
    for request in card_line_request_reader.read() {
        match &request.request_type {
            CardLineRequestType::AddToLine { card_entity } => {
                if let Ok(mut card_line) = card_lines.get_mut(request.line) {
                    let card_inserted = add_card_to_line_if_in_capacity(
                        *card_entity,
                        request.line,
                        &mut card_line,
                        &mut cards,
                        debug_logs_enabled.0,
                        &mut commands,
                    );
                    if !card_inserted {
                        return;
                    }
                }
            }
            CardLineRequestType::BatchAddToLine { card_entities } => {
                if let Ok(mut card_line) = card_lines.get_mut(request.line) {
                    for card_entity in card_entities {
                        let card_inserted = add_card_to_line_if_in_capacity(
                            *card_entity,
                            request.line,
                            &mut card_line,
                            &mut cards,
                            debug_logs_enabled.0,
                            &mut commands,
                        );
                        if !card_inserted {
                            return;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn add_card_to_line_if_in_capacity(
    card_entity: Entity,
    card_line_entity: Entity,
    card_line: &mut CardLine,
    cards: &mut Query<(&mut Card, &Name)>,
    should_log: bool,
    commands: &mut Commands,
) -> ActionPerformed {
    let mut card_name_if_added = None;
    let card_inserted = card_line.push_if_theres_space(card_entity);
    if card_inserted.done() {
        commands.entity(card_line_entity).add_child(card_entity);
        if let Ok((mut card, name)) = cards.get_mut(card_entity) {
            card.owner_line = Some(card_line_entity);
            card_name_if_added = Some(name.clone());
        }
        if should_log {
            if let Some(card_name) = card_name_if_added {
                let mut names_in_order = vec![];
                for card_entity in card_line.cards_in_order() {
                    if let Ok((_, name)) = cards.get(*card_entity) {
                        names_in_order.push(name);
                    }
                }
                info!(
                    "{} was added to a card line, so it's now: {:?}",
                    card_name, names_in_order
                );
            }
        }
    }
    card_inserted
}

fn listen_to_dragged_card_movements(
    moved_dragged_cards: Query<(&Transform, &Card, &Dragged, Entity), Changed<Transform>>,
    mut card_lines: Query<(&mut CardLine, &Transform)>,
    cards: Query<(&Card, &Name)>,
    debug_logs_enabled: Res<CardsPluginShouldPrintLogs>,
) {
    for (dragged_card_transform, dragged_card, card_dragged_component, dragged_card_entity) in
        &moved_dragged_cards
    {
        if let Dragged::GoingBackToPlace = card_dragged_component {
            continue;
        }
        if let Some(card_line_entity) = dragged_card.owner_line {
            if let Ok((mut card_line, card_line_transform)) = card_lines.get_mut(card_line_entity) {
                sort_on_dragged_card_movement(
                    dragged_card_transform,
                    dragged_card_entity,
                    &cards,
                    &mut card_line,
                    card_line_transform,
                    debug_logs_enabled.0,
                );
            }
        }
    }
}

fn sort_on_dragged_card_movement(
    dragged_card_transform: &Transform,
    dragged_card_entity: Entity,
    cards: &Query<(&Card, &Name)>,
    owner_card_line: &mut CardLine,
    card_line_transform: &Transform,
    debug_logs_enabled: bool,
) {
    let distance_from_origin_signed = projection_directed_distance(
        dragged_card_transform.translation.xy(),
        card_line_transform.right().xy(),
        card_line_transform.translation.xy(),
    );
    let mut maybe_new_dragged_card_index = None;
    let mut maybe_dragged_card_index = None;
    for (card_index, card_entity) in owner_card_line.cards_in_order().iter().enumerate() {
        if let Ok((card, _)) = cards.get(*card_entity) {
            if (card.origin.translation.x - distance_from_origin_signed).abs()
                < owner_card_line.card_origin_gap * (2.0 / 5.0)
            {
                maybe_new_dragged_card_index = Some(card_index);
            }
        }
        if *card_entity == dragged_card_entity {
            maybe_dragged_card_index = Some(card_index);
        }
    }
    if maybe_new_dragged_card_index.is_none() {
        let first_card_distance_from_center =
            calculate_first_card_distance_from_center(owner_card_line);
        if distance_from_origin_signed < -first_card_distance_from_center {
            maybe_new_dragged_card_index = Some(0);
        } else if distance_from_origin_signed > first_card_distance_from_center {
            maybe_new_dragged_card_index = Some(owner_card_line.cards_in_order().len() - 1);
        }
    }

    if let (Some(dragged_card_index), Some(dragged_card_new_index)) =
        (maybe_dragged_card_index, maybe_new_dragged_card_index)
    {
        let swapped = owner_card_line.heavy_swap(dragged_card_index, dragged_card_new_index);

        if debug_logs_enabled && swapped.0 {
            let mut names_in_order = vec![];
            for card_entity in owner_card_line.cards_in_order() {
                if let Ok((_, name)) = cards.get(*card_entity) {
                    names_in_order.push(name);
                }
            }
            info!(
                "A dragged card moved from index {} to index {} making the card line: {:?}",
                dragged_card_index, dragged_card_new_index, names_in_order
            );
        }
    }
}

fn set_card_origins_on_line_change(
    changed_card_lines: Query<&CardLine, Changed<CardLine>>,
    mut cards: Query<&mut Card>,
) {
    for card_line in &changed_card_lines {
        let first_card_x = -1.0 * calculate_first_card_distance_from_center(card_line);
        for (index, card_entity) in card_line.cards_in_order().iter().enumerate() {
            if let Ok(mut card) = cards.get_mut(*card_entity) {
                let resulting_translation = card
                    .origin
                    .translation
                    .with_x(first_card_x + index as f32 * card_line.card_origin_gap);
                card.origin.translation = resulting_translation;
            }
        }
    }
}

fn calculate_first_card_distance_from_center(card_line: &CardLine) -> f32 {
    let location_count = card_line.cards_in_order().len();
    let card_origin_gap = card_line.card_origin_gap;
    if location_count % 2 == 1 {
        ((location_count - 1) / 2) as f32 * card_origin_gap
    } else {
        ((location_count / 2) as f32 * card_origin_gap) - (card_origin_gap / 2.0)
    }
}
