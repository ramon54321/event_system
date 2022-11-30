use events::{EnumDiscriminants, EnumIndex, EnumIndexGet, Events};

#[repr(u16)]
#[derive(EnumIndex, EnumDiscriminants)]
#[strum_discriminants(repr(u16))]
#[strum_discriminants(derive(EnumIndex))]
#[strum_discriminants(name(EventTag))]
enum Event {
    ApplicationLoad,
    ApplicationExit,
    KeyDown(u8),
}

fn main() {
    let mut events = Events::<Event, EventTag>::new(64);

    events.register(EventTag::ApplicationLoad, on_application_load);
    events.register(EventTag::ApplicationLoad, || {
        println!("After load");
        true
    });

    events.fire(Event::ApplicationLoad);
}

fn on_application_load() -> bool {
    println!("Application Loaded");
    true
}

fn on_application_exit() -> bool {
    println!("Application Exit");
    true
}
