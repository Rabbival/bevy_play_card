use bevy_play_card::prelude::*;

const MAX_PICKED_CARDS_IN_LINE: usize = 3;
const MAX_CARDS_IN_LINE: usize = MAX_PICKED_CARDS_IN_LINE * 2;

type CardIsPicked = bool;

#[derive(Debug)]
struct PickedStates {
    before_policy_trigger: Vec<CardIsPicked>,
    after_policy_trigger: Vec<CardIsPicked>,
}

#[test]
fn test_picked_cards_policy() {
    let mut app = App::new();

    app.init_resource::<Time>()
        .add_plugins(BevyCardPlugin {
            // card_debug_logging_function: Some(log),
            ..default()
        })
        .add_systems(Startup, spawn_lines);

    let mut picked_states_by_policy: HashMap<CardPickingPolicy, PickedStates> = HashMap::new();
    let picks_expected_before_policy_triggers = [true, true, true, false, false, false];
    let picks_expected_after_policy_triggers: HashMap<CardPickingPolicy, [CardIsPicked; 6]> =
        HashMap::from([
            (
                CardPickingPolicy::ForbidNewOnes,
                [true, true, true, false, false, false],
            ),
            (
                CardPickingPolicy::RemoveMostRecentlyPicked,
                [true, true, false, false, false, true],
            ),
            (
                CardPickingPolicy::RemoveLeastRecentlyPicked,
                [false, false, false, true, true, true],
            ),
        ]);

    app.update();

    app.add_systems(Update, request_first_half_picks.run_if(run_once));

    app.update();
    app.update();

    let lines_query: Vec<(&CardLine, &PolicyTag)> = app
        .world_mut()
        .query::<(&CardLine, &PolicyTag)>()
        .iter(app.world())
        .collect();
    let lines: Vec<(CardLine, PolicyTag)> = lines_query
        .iter()
        .map(|(line, policy)| ((*line).clone(), **policy))
        .collect::<Vec<_>>();

    for (line, policy_tag) in &lines {
        picked_states_by_policy.insert(
            policy_tag.0,
            PickedStates {
                before_policy_trigger: cards_picked_state_in_order(line, &app),
                after_policy_trigger: Vec::new(),
            },
        );
    }

    app.add_systems(Update, request_last_half_picks.run_if(run_once));

    app.update();
    app.update();

    for (line, policy_tag) in &lines {
        picked_states_by_policy
            .get_mut(&policy_tag.0)
            .unwrap()
            .after_policy_trigger = cards_picked_state_in_order(line, &app);
    }

    for (_, policy_tag) in &lines {
        let before_policy = &picked_states_by_policy
            .get_mut(&policy_tag.0)
            .unwrap()
            .before_policy_trigger;

        for i in 0..MAX_CARDS_IN_LINE {
            assert_eq!(picks_expected_before_policy_triggers[i], before_policy[i]);
        }
    }
    for (_, policy_tag) in &lines {
        let expected_after_policy = picks_expected_after_policy_triggers
            .get(&policy_tag.0)
            .unwrap();
        let after_policy = &picked_states_by_policy
            .get_mut(&policy_tag.0)
            .unwrap()
            .after_policy_trigger;

        for i in 0..MAX_CARDS_IN_LINE {
            assert_eq!(expected_after_policy[i], after_policy[i]);
        }
    }
}

// fn log(log_me: String) {
//     println!("{}", log_me);
// }

fn spawn_lines(mut card_line_request_writer: EventWriter<CardLineRequest>, mut commands: Commands) {
    for policy in [
        CardPickingPolicy::ForbidNewOnes,
        CardPickingPolicy::RemoveMostRecentlyPicked,
        CardPickingPolicy::RemoveLeastRecentlyPicked,
    ] {
        let line = commands
            .spawn((
                CardLineBundle::from_card_line(
                    CardLine::default()
                        .with_max_cards(MAX_CARDS_IN_LINE)
                        .with_picked_cards_capacity(MAX_PICKED_CARDS_IN_LINE)
                        .with_card_picking_policy(policy),
                ),
                PolicyTag(policy),
            ))
            .id();
        let mut card_entities = vec![];
        for _ in 0..MAX_CARDS_IN_LINE {
            card_entities.push(commands.spawn(CardBundle::new(Transform::default())).id());
        }
        card_line_request_writer.write(CardLineRequest {
            line,
            request_type: CardLineRequestType::BatchAddToLine { card_entities },
        });
    }
}

fn request_first_half_picks(
    mut requester: EventWriter<TogglePickingForCard>,
    lines: Query<&CardLine>,
) {
    for line in &lines {
        if let Some(card_entities) = line
            .cards_in_order()
            .first_chunk::<MAX_PICKED_CARDS_IN_LINE>()
        {
            requester.write_batch(card_entities.map(|entity| TogglePickingForCard(entity)));
        }
    }
}

fn request_last_half_picks(
    mut requester: EventWriter<TogglePickingForCard>,
    lines: Query<&CardLine>,
) {
    for line in &lines {
        if let Some(card_entities) = line
            .cards_in_order()
            .last_chunk::<MAX_PICKED_CARDS_IN_LINE>()
        {
            requester.write_batch(card_entities.map(|entity| TogglePickingForCard(entity)));
        }
    }
}

fn cards_picked_state_in_order(line: &CardLine, app: &App) -> Vec<CardIsPicked> {
    let mut picked_state = Vec::new();
    for &card_entity in line.cards_in_order() {
        picked_state.push(app.world().get::<Picked>(card_entity).is_some());
    }
    picked_state
}

#[derive(Component, Debug, Clone, Copy)]
struct PolicyTag(CardPickingPolicy);
