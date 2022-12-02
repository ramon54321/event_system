use events::create_event_system;

create_event_system! {
    EventSystem
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
