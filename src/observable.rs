use crossbeam_queue::SegQueue;
use slotmap::{new_key_type, DenseSlotMap};

new_key_type! {pub struct ObservationKey;}

pub struct Observable<'a, Event> {
    observers: DenseSlotMap<ObservationKey, Box<dyn Fn(&Event) + 'a>>,
    event_queue: SegQueue<Event>,
}

impl<'a, Event> Observable<'a, Event> {
    pub fn new() -> Self {
        return Self {
            observers: DenseSlotMap::with_key(),
            event_queue: SegQueue::new(),
        };
    }

    pub fn observe<F: Fn(&Event) + 'a>(&mut self, subscriber: F) -> ObservationKey {
        self.observers.insert(Box::new(subscriber))
    }
    }

    pub fn publish(&self, event: Event) {
        self.event_queue.push(event);
    }

    pub fn publish_and_emit(&self, event: Event) {
        self.event_queue.push(event);

        self.emit_all();
    }

    fn emit_all(&self) {
        while !self.event_queue.is_empty() {
            self.emit_once();
        }
    }

    fn emit_once(&self) -> bool {
        self.event_queue
            .pop()
            .map(|event| self.emit_event(&event))
            .is_some()
    }

    pub fn emit_event(&self, event: &'a Event) {
        for observer in self.observers.values() {
            observer.try_call(event);
        }
    }

    pub fn map<NewEvent, F: Fn(&Event) -> NewEvent + 'a>(
        &self,
        map: F,
    ) -> Observable<'a, NewEvent> {
        todo!();
    }
}
