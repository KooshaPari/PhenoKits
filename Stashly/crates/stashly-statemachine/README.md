# stashly-statemachine

Generic finite state machine with transition guards and callbacks.

**Features:**
- Type-safe state transitions
- Guard functions (pre-transition validation)
- Transition callbacks (post-transition hooks)
- Generic over state and event types
- No panics: guard failures return errors

**Tests:** Inline `#[test]` in `src/lib.rs`

**Usage:**
```rust
let mut fsm = StateMachine::new(State::Init);
fsm.transition(Event::Start, guard_fn)?;
assert_eq!(fsm.current_state(), State::Running);
```
