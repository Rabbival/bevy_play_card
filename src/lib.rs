#![allow(clippy::type_complexity)]
/*!
## Using The Crate

### Registering the Plugin
First, you should add `BevyCardPlugin`, it has a `::default()` implementation but can be reconfigured, for example:
  ```rust
  app.add_plugins(
    BevyCardPlugin {
        card_consts: CardConsts {
            card_hover_height: 40.0,
            ..default()
        },
        ..default()
    },
  );
  ```

### Spawning Cards
The basic functionality derived from Bevy Picking is added to Cards automatically.
It's recommended to instantiate a card using card bundles:
  ```rust
  commands.spawn((
    CardBundle::new(Transform::default()),
    Sprite {
        image: asset_server.load("your card sprite path here"),
        ..default ()
    },
  ));
  ```
Cards are named automatically by the card_namer.
When the `Card` component is removed from an entity, it's automatically removed from it's owner line if it had any.

### Spawning Card Lines
Cards can live on a card line, that'll allow you to reorder them and keep them neatly organized.
All you need to instantiate a card line is:
  ```rust
  commands
    .spawn(CardLineBundle::from_transform(Transform::default()));
```
Or more specifically using `from_card_line`:
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

### Using Card Line Requests
Although you could interact with the entities directly, you can spare yourself some boilerplate
by firing events using `EventWriter<CardLineRequest>`, already listened to by the plugin.
Each `CardLineRequest` contains the entity of the line to which it'll be applied and a request type with relevant additional data if there's any.
The variants of `CardLineRequestType` are as follows:

| Variant                  | Role                                                                      |
|--------------------------|---------------------------------------------------------------------------|
| `RaiseLine`              | Raises the card line to it's relative up                                  |
| `LowerLine`              | Lowers the card line back to place after being raised                     |
| `AddToLine`              | Inserts the card to the line if within capacity                           |
| `RemoveFromLine`         | Removes the card from the line if it was there                            |
| `BatchAddToLine`         | Inserts the cards to the line if within capacity, starting from the first |
| `BatchRemoveFromLine`    | Removes the cards from the line if they were there                        |
| `RemoveAllCardsFromLine` | Removes all the cards in the line                                         |

### Card Tags
Cards are being tagged for easier queries when they're hovered, picked and dragged.
`Hovered` and `Dragged` are removed once the status is done, `Picked` is toggled by clicking.
This way you can, for example, get all hovered card with queries like:
`
  Query<&Card, With<Hovered>>
`

| Tag       | Visually                                                                                      |
|-----------|-----------------------------------------------------------------------------------------------|
| `Hovered` | The card would hover up a bit                                                                 |
| `Dragged` | The card would follow your cursor, ignoring line movements, going back to place once released |
| `Picked`  | The card would stay on hover height, ignoring the cursor leaving its area                     |


### Workflow Example
Let's spawn a line and add a few cards to it,
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
    request_type: CardLineRequestType::AddToLine { card_entity },
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
      request_type: CardLineRequestType::AddToLine { card_entity },
    });
  }
  ```
*/

pub mod cards;
mod generic_plugins;
pub mod trait_unions;

#[macro_use]
pub mod macros;
pub mod bevy_card_plugin;
pub mod utilities;

pub mod prelude {
    pub use crate::bevy_card_plugin::*;
    pub use crate::cards::{
        CardsPlugin, CardsPluginLoggingFunction,
        card::*,
        card_bundle::*,
        card_lines::{
            CardLinesPlugin, card_line::*, card_line_bundle::*, card_lines_content_manager::*,
            card_lines_mover::*, event::*,
        },
        card_managers::{
            CardManagersPlugin, card_observer_attacher::*, card_origin_set_listener::*,
            card_tag_insertion_listener::*, theres_an_actively_dragged_card_from_that_line,
        },
        card_namer::*,
        event::*,
        tags::*,
    };
    pub use crate::trait_unions::*;
    pub use crate::utilities::system_sets::*;
    pub use bevy::{platform::collections::HashMap, prelude::*};
    pub use bevy_tween_helpers::*;
    pub use std::marker::PhantomData;
}
