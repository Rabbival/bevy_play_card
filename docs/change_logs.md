## Change Logs

[(Click me to go back to the main readme)](../README.md)

#### '0.3.0' -> '0.4.0'
* Add `CardPickingPolicy` and `CardPickingPolicyWithContent` to card lines 
* Fix card dragging never ends bug

#### '0.2.7' -> '0.3.0'
* Extract tween utilities logic into its own `bevy_tween_helpers` crate

#### `0.2.6 -> 0.2.7`
* Add observers per card (instead of to the entire app) to allow for pointer trigger propagations
* Update [multiple_card_lines.rs](../examples/multiple_card_lines.rs) example to use `CardConsts` (see 0.2.4 changes)

#### `0.2.4 -> 0.2.6`
* Update crate to build docs.rs

#### `0.2.3 -> 0.2.4`
* Enable providing logging functions for cards and tweening for more flexible debugging
* `CardConsts` is now a field within `BevyCardPlugin` (instead of having duplicate fields)
* `card_drag_delta_scaler` field was added to `CardConsts` to account for camera projection changes
* When other cards make space for the dragged card, they now accounts for changes in the `CardLine` transform's scale

#### `0.2.2 -> 0.2.3`
* Fix broken docs links
* Add two examples:
  * [disposable_cards_plus.rs](../examples/disposable_cards_plus.rs) for enhancement using [`bevy_tween`](https://github.com/Multirious/bevy_tween)
  * [arced_cards.rs](../examples/arced_cards.rs) for both additions and functionality overrides

#### `0.2.1 -> 0.2.2`
* Add `at_capacity` function to card lines to reduce boilerplate

#### `0.2 -> 0.2.1`
* Make hovering over cards while dragging configurable, 
  so that you can forbid hovering over cards from all lines while a card is being actively dragged

### `0.1 -> 0.2`
* (Possibly Breaking) Changes:
  * Update from Bevy 0.15 to Bevy 0.16
  * Card animation-managers-and-taggers moved under [`cards_managers`](../src/cards/card_managers) folder (and plugin)
  * Make [TweenPriorityToOthersOfType](../src/tweening/tween_priority.rs) a u32 and change the ones created in the crate to have gaps between them
  * Tweens of the same type now don't destroy each other if they have a shared parent entity, which enables same-type-sequences
  * Shorten [`LineRequestType`](../src/cards/card_lines/event.rs) variants:
    * `RaiseCardLine -> RaiseLine`
    * `LowerCardLine -> LowerLine`
    * `AddToCardLine -> AddToLine`
    * `RemoveFromCardLine -> RemoveFromLine`


* Additions:
  * Add `Picked` cards tag on click, remove them on the next one. Lines may have a capacity for those.
  * Add an observer that removes cards from their owner line when their `Card` component is removed
  * Add `BatchAddToLine, BatchRemoveFromLine, RemoveAllCardsFromLine` request types to [`CardLineRequest`](../src/cards/card_lines/event.rs)
  * The [`ActionPerformed`](../src/utilities/action_performed.rs) wrapper can now be negated (for example `let whatever = ActionPerformed(true);` and then `!whatever`)
  * Make card naming run possibly infinitely, and would only name unnamed cards


* Bug Fixes:
  * Cards snapping back to origin if not hovered when dragged
  * Some possible crashes from using `despawn()` instead of `try_despawn()`

[(Click me to go back to the main readme)](../README.md)