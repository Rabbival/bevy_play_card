## Change Logs

[(Click me to go back to the main readme)](../README.md)

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