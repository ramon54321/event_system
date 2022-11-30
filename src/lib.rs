use std::marker::PhantomData;

pub use enum_index_repr::{EnumIndex, EnumIndexGet};
pub use strum::EnumDiscriminants;

pub struct Events<E, T> {
    capacity: usize,
    listeners: Vec<Vec<fn() -> bool>>,
    _phantom_e: PhantomData<E>,
    _phantom_t: PhantomData<T>,
}
impl<E, T> Events<E, T>
where
    E: EnumIndexGet,
    T: EnumIndexGet,
{
    ///
    /// Create new events system with the given capacity of event types.
    ///
    /// Capacity needs to exceed number of variants of `T`.
    ///
    pub fn new(capacity: usize) -> Self {
        let mut listeners = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            listeners.push(Vec::new());
        }
        Self {
            capacity,
            listeners,
            _phantom_e: PhantomData::default(),
            _phantom_t: PhantomData::default(),
        }
    }

    ///
    /// Register callback to event.
    ///
    /// Multiple callbacks can be registered to the same event and will be called in sequence of
    /// registration, breaking early if a callback returns `true`.
    ///
    pub fn register(&mut self, event_tag: T, callback: fn() -> bool) {
        let tag_index = event_tag.index() as usize;

        assert!(
            tag_index < self.capacity,
            "Can not register callback to events: Capacity exceeded."
        );

        self.listeners[tag_index].push(callback);
    }

    ///
    /// Fire the given event.
    ///
    pub fn fire(&self, event: E) {
        let tag_index = event.index() as usize;

        assert!(
            tag_index < self.capacity,
            "Can not fire event: Event index exceeds capacity."
        );

        let callbacks = &self.listeners[tag_index];

        for callback in callbacks {
            if callback() {
                return;
            }
        }
    }
}
