use slotmap::{new_key_type, DenseSlotMap};

new_key_type! {pub struct ObservationKey;}

pub struct Emitter<'a, Event> {
    observers: DenseSlotMap<ObservationKey, Box<dyn FnMut(&Event) + 'a>>,
}

impl<'a, Event> Emitter<'a, Event> {
    pub fn new() -> Self {
        Self {
            observers: DenseSlotMap::with_key(),
        }
    }

    pub fn observe<F: FnMut(&Event) + 'a>(&mut self, subscriber: F) -> ObservationKey {
        self.observers.insert(Box::new(subscriber))
    }

    pub fn emit(&mut self, event: &Event) {
        for observer in self.observers.values_mut() {
            observer(event);
        }
    }
}
