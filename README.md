# bevy_card
### A card crate for the Bevy game engine

## How to Use
### Registering the Plugin
First, you should add `BevyCardPlugin`, it has a `::default()` implementation but can be reconfigured, for example:
  ```rust
  app.add_plugins(
    BevyCardPlugin {
    card_hover_height: 40.0,
    ..default()
    },
  );
  ```

### Spawning Cards
The basic functionality derived from Bevy Picking is added to Cards automatically. 
All you need to do to instantiate a card is:
  ```rust
  commands.spawn((
    CardBundle::new(Transform::default()),
    Sprite {
        image: asset_server.load("your card sprite path here"),
        ..default ()
    },
  ));
  ```

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
Although you could interact with the entities directly, 
a lot of the boilerplate is handled by `EventReader<CardLineRequest>`listeners already registered by the plugin.
Each `CardLineRequest` contains the entity of the line to which it'll be applied and a request type with relevant additional data if there's any.
The variants of `CardLineRequestType` are as follows:

| Variant              | Role                                                  |
|----------------------|-------------------------------------------------------|
| `RaiseCardLine`      | Raises the card line to it's relative up              |
| `LowerCardLine`      | Lowers the card line back to place after being raised |
| `AddToCardLine`      | Inserts the card to the line if possible              |
| `RemoveCardFromLine` | Removes the card from the line if it was there        |


## Bevy Version Support
| `bevy` | `bevy_tween` |
|--------|--------------|
| 0.15   | 0.1          |

## Credits
- [`bevy_tween`](https://github.com/Multirious/bevy_tween)
  The crate this one is built upon. Thanks to it I was able to set bevy_card up rather quickly and easily.