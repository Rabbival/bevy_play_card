# bevy_play_card
### A card crate for the [`Bevy`](https://bevyengine.org/) game engine

![bevy_card_showcase - Trim](https://github.com/user-attachments/assets/bf92b236-76ce-4beb-b929-eec4c85ce166)

## How to Use
### Registering the [Plugin](src/bevy_card_plugin.rs)
First, you should add [`BevyCardPlugin`](src/bevy_card_plugin.rs), it has a `::default()` implementation but can be reconfigured, for example:
  ```rust
  app.add_plugins(
    BevyCardPlugin {
      card_hover_height: 40.0,
      ..default()
    },
  );
  ```

### Spawning [Cards](src/cards/card.rs)
The basic functionality derived from Bevy Picking is added to Cards automatically. 
It's recommended to instantiate a card using [card bundle](src/cards/card_bundle.rs)s:
  ```rust
  commands.spawn((
    CardBundle::new(Transform::default()),
    Sprite {
        image: asset_server.load("your card sprite path here"),
        ..default ()
    },
  ));
  ```
Cards are named automatically by the [card_namer](src/cards/card_namer.rs).

### Spawning [Card Lines](src/cards/card_lines/card_line.rs)
Cards can live on a card line, that'll allow you to reorder them and keep them neatly organized. 
All you need to instantiate a card line is:
  ```rust
  commands
    .spawn(CardLineBundle::from_transform(Transform::default()));
```
Or more specifically using [`from_card_line`](src/cards/card_lines/card_line_bundle.rs):
  ```rust
  commands
    .spawn(CardLineBundle::from_card_line(
        CardLine::default()
          .with_origin(
            Transform::from_translation(location.into())
                .with_rotation(Quat::from_rotation_z(rotation)),
          )
          .with_max_cards(MAX_CARDS)
          .with_card_origin_gap(60.0),
    ));
  ```

### Using [Card Line Requests](src/cards/card_lines/event.rs)
Although you could interact with the entities directly, you can spare yourself some boilerplate
by firing events using `EventWriter<CardLineRequest>`, already listened to by the plugin.
Each [`CardLineRequest`](src/cards/card_lines/event.rs) contains the entity of the line to which it'll be applied and a request type with relevant additional data if there's any.
The variants of `CardLineRequestType` are as follows:

| Variant              | Role                                                  |
|----------------------|-------------------------------------------------------|
| `RaiseCardLine`      | Raises the card line to it's relative up              |
| `LowerCardLine`      | Lowers the card line back to place after being raised |
| `AddToCardLine`      | Inserts the card to the line if possible              |
| `RemoveCardFromLine` | Removes the card from the line if it was there        |

### Hovered And Dragged [Tags](src/cards/tags.rs)
Cards are being tagged for easier queries when they're hovered and dragged (the tags are removed when the action is done).
This way you can, for example, get all hovered card with queries like:
`
  Query<&Card, With<Hovered>>
`
For use examples, see [using_hovered_and_dragged_cards.rs](examples/using_hovered_and_dragged_cards.rs)

### Workflow Example
Let's spawn a line and add a few cards to it, 
see [cards_on_a_line.rs](examples/cards_on_a_line.rs) for another example of that scenario.

First, let's spawn a defaultly-configured line in the middle of the screen.
  ```rust
  let line_entity = commands
    .spawn(CardLineBundle::from_transform(Transfrom::default())).id();
  ```

Now, let's spawn three cards, sending a request to add each to a line. 
Sending the requests separately ensures the line stays within capacity, leaving out the last cards to be spawned if any.
The shape of the loop is as follows:
  ```rust
  for _ in 0..3 {
    let card_entity = commands.spawn(( ... )).id();
    card_line_request_writer.write(CardLineRequest { ... });
  }
  ```

An example for card spawning would be:
  ```rust
  let card_entity = commands
    .spawn((
        Sprite {
          image: asset_server.load("path to your card sprite here"),
          ..default()
        },
        CardBundle::new(Transform::default()),
      )).id();
```
With the appropriate addition request:
```rust
  card_line_request_writer.write(CardLineRequest {
    line: line_entity,
    request_type: CardLineRequestType::AddToCardLine { card_entity },
  });
  ```

Ending up with:
  ```rust
  let line_entity = commands
    .spawn(CardLineBundle::from_transform(Transfrom::default())).id();
  for _ in 0..3 {
    let card_entity = commands
        .spawn((
          Sprite {
            image: asset_server.load("path to your card sprite here"),
            ..default()
          },
          CardBundle::new(Transform::default()),
        )).id();
    card_line_request_writer.write(CardLineRequest {
      line: line_entity,
      request_type: CardLineRequestType::AddToCardLine { card_entity },
    });
  }
  ```

## Bevy Version Support
| `bevy` | `bevy_tween` |
|--------|--------------|
| 0.15   | 0.1          |

## Credits
- [`bevy_tween`](https://github.com/Multirious/bevy_tween)
  The crate this one is built upon. Thanks to it I was able to set bevy_card up rather quickly and easily.
