## Tween Priorities To Others Of Type

[(Click me to go back to the main readme)](../README.md)

See [TweenPriorityToOthersOfType](../src/tweening/tween_priority.rs)

| Tween Name                           | Tween Use                        | Tween Type                | Priority |
|--------------------------------------|----------------------------------|---------------------------|----------|
| **on-picked**                        | A card has been picked           | `Translation` and `Scale` | **50**   |
| **new-origin-set**                   | A card's origin has been updated | `Translation` and `Scale` | **40**   |
| **go-back-to-origin-after-dragging** | A dragged card was released      | `Translation` and `Scale` | **30**   |
| **on-hover-cancel**                  | A card is no longer hovered      | `Translation` and `Scale` | **20**   |
| **card line slide**                  | A card-line should be moving     | `Translation`             | **20**   |
| **on-hover**                         | A card is now hovered            | `Translation` and `Scale` | **10**   |


[(Click me to go back to the main readme)](../README.md)