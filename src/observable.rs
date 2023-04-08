use crossbeam_queue::SegQueue;

pub struct Observable<'a, Event> {
    observers: Vec<Box<dyn Fn(&Event) + 'a>>,
    event_queue: SegQueue<Event>,
}

impl<'a, Event> Observable<'a, Event> {
    pub fn new() -> Self {
        return Self {
            observers: Vec::new(),
            event_queue: SegQueue::new(),
        };
    }

    pub fn observe<F: Fn(&Event) + 'a>(&mut self, observer: F) {
        self.observers.push(Box::new(observer));
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
        for obs in self.observers.iter() {
            obs(event);
        }
    }

    pub fn map<NewEvent, F: Fn(&Event) -> NewEvent + 'a>(
        &self,
        map: F,
    ) -> Observable<'a, NewEvent> {
        todo!();
    }
}
