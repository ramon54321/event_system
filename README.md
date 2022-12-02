<div align="center">
    <span><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1920px-Rust_programming_language_black_logo.svg.png" width="100"></span>
</div>

### Event System 

Provides a simple lightweight event system for Rust. 

[Crate on Crates.io](https://crates.io/crates/events_system)

### Example

The first entry in the macro is the name of the event system struct.

Any following entries take the form of struct-enum variants, where the name is the event name and the data the structure for the payload when the event is fired.

```rust
use events::create_event_system;

create_event_system! {
    EventSystem // <- Name of struct
    ApplicationLoad {}
    ApplicationExit {}
    KeyDown {
        key: u8,
    }
}

fn main() {
    let mut events = EventSystem::new();
    events.register_key_down(key_down);

    events.fire_application_load();
    events.fire_key_down(EventKeyDown { key: 55 });
    events.fire_application_exit();
}

fn key_down(packet: EventKeyDown) -> bool {
    println!("Key down {:?}", packet.key);
    true
}
```

