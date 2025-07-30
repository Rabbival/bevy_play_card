use crate::prelude::*;
use bevy::ecs::relationship::OrderedRelationshipSourceCollection;

pub struct CardPickingPlugin;

impl Plugin for CardPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_to_picking_toggle_requests);
    }
}

fn listen_to_picking_toggle_requests(
    mut request_listener: EventReader<TogglePickingForCard>,
    picked_cards: Query<&Card, With<Picked>>,
    dragged_cards: Query<(&Card, &Dragged)>,
    cards: Query<&Card>,
    mut card_lines: Query<&mut CardLine>,
    mut commands: Commands,
) {
    for TogglePickingForCard(card_entity) in request_listener.read(){
        handle_picking_request(
            *card_entity,
            &picked_cards,
            &dragged_cards,
            &cards,
            &mut card_lines,
            &mut commands,
        );
    }
}

pub(crate) fn on_card_click(
    trigger: Trigger<Pointer<Click>>,
    picked_cards: Query<&Card, With<Picked>>,
    dragged_cards: Query<(&Card, &Dragged)>,
    cards: Query<&Card>,
    mut card_lines: Query<&mut CardLine>,
    mut commands: Commands,
) {
    handle_picking_request(
        trigger.target(),
        &picked_cards,
        &dragged_cards,
        &cards,
        &mut card_lines,
        &mut commands,
    );
}

fn handle_picking_request(
    card_entity: Entity,
    picked_cards: &Query<&Card, With<Picked>>,
    dragged_cards: &Query<(&Card, &Dragged)>,
    cards: &Query<&Card>,
    card_lines: &mut Query<&mut CardLine>,
    commands: &mut Commands,
) {
    if let Ok(card) = cards.get(card_entity) {
        let card_is_picked = picked_cards.contains(card_entity);
        if let Ok(mut card_entity_commands) = commands.get_entity(card_entity) {
            if card_is_picked {
                card_entity_commands.remove::<Picked>();
            } else if dragged_cards.contains(card_entity) {
                return;
            } else if let Some(owner_line_entity) = card.owner_line
                && let Ok(mut card_line) = card_lines.get_mut(owner_line_entity)
                && let Some(picked_cards_capacity) = card_line.picked_cards_capacity
            {
                if picked_cards_capacity == 0 {
                    return;
                }
                handle_picking_by_owner_line_policy(
                    card_entity,
                    &mut card_line,
                    picked_cards_capacity,
                    &picked_cards,
                    commands,
                );
            } else {
                card_entity_commands.insert(Picked);
            }
        }
    }
}

fn handle_picking_by_owner_line_policy(
    card_entity: Entity,
    card_line: &mut CardLine,
    picked_cards_capacity: usize,
    picked_cards: &Query<&Card, With<Picked>>,
    commands: &mut Commands,
) {
    let picked_cards_in_line = picked_cards.iter().filter(|card| match card.owner_line {
        Some(owner_line_entity) => owner_line_entity == owner_line_entity,
        None => false,
    });
    let card_line_at_picked_capacity = picked_cards_in_line.count() >= picked_cards_capacity;
    match &mut card_line.picked_card_policy {
        CardPickingPolicyWithContent::ForbidNewOnes => {
            if card_line_at_picked_capacity {
                return;
            }
        }
        CardPickingPolicyWithContent::RemoveLeastRecentlyPicked {
            picked_cards_in_order,
        } => {
            if card_line_at_picked_capacity {
                while let Some(last_picked) = picked_cards_in_order.remove_at_stable(0) {
                    if picked_cards.contains(last_picked) //could be that it's no longer picked despite being registered
                        && let Ok(mut card_commands) = commands.get_entity(last_picked)
                    {
                        card_commands.remove::<Picked>();
                        break;
                    }
                }
            }
            picked_cards_in_order.push(card_entity);
        }
        CardPickingPolicyWithContent::RemoveMostRecentlyPicked(newest_picked) => {
            if card_line_at_picked_capacity
                && let Some(picked_card) = newest_picked
                && picked_cards.contains(*picked_card)
                && let Ok(mut card_commands) = commands.get_entity(*picked_card)
            {
                card_commands.remove::<Picked>();
            }
            *newest_picked = Some(card_entity);
        }
    }
    commands.entity(card_entity).insert(Picked);
}

pub(crate) fn remove_hovered_on_picked_removal(
    trigger: Trigger<OnRemove, Picked>,
    cards: Query<(), With<Card>>,
    mut commands: Commands,
) {
    if cards.contains(trigger.target())
        && let Ok(mut card_commands) = commands.get_entity(trigger.target())
    {
        card_commands.remove::<Hovered>();
    }
}
