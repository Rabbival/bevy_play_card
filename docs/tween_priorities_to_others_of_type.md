## Tween Priorities To Others Of Type

[(Click me to go back to the main readme)](../README.md)

See [TweenPriorityToOthersOfType](https://github.com/Rabbival/bevy_tween_helpers/blob/main/src/tween_priority.rs)

| Tween Name                           | Tween Use                                        | Tween Type                | Priority |
|--------------------------------------|--------------------------------------------------|---------------------------|----------|
| **new-origin-set**                   | A card's origin has been updated                 | `Translation` and `Scale` | **40**   |
| **go-back-to-origin-after-dragging** | A dragged card was released                      | `Translation` and `Scale` | **30**   |
| **card line slide**                  | A card-line should be moving                     | `Translation`             | **20**   |
| **float-back-down**                | A card is no longer hovered or is being unpicked | `Translation` and `Scale` | **10** (see note)   |
| **on-hover**                      | A card is now hovered                            | `Translation` and `Scale` | **10** (see note)   |

note: Priority turns to 50 if sliding to a new location

[(Click me to go back to the main readme)](../README.md)
