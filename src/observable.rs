use crossbeam_queue::SegQueue;

pub struct Observable<'a, Item> {
    observers: Vec<Box<dyn Fn(&Item) + 'a>>,
    event_queue: SegQueue<Item>,
}

impl<'a, Item> Observable<'a, Item> {
    pub fn new() -> Self {
        return Self {
            observers: Vec::new(),
            event_queue: SegQueue::new(),
        };
    }

    pub fn observe<F: Fn(&Item) + 'a>(&mut self, observer: F) {
        self.observers.push(Box::new(observer));
    }

    pub fn publish(&self, item: Item) {
        self.event_queue.push(item);
    }

    pub fn publish_and_emit(&self, item: Item) {
        self.event_queue.push(item);

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
            .map(|item| self.emit_item(&item))
            .is_some()
    }

    pub fn emit_item(&self, item: &'a Item) {
        for obs in self.observers.iter() {
            obs(item);
        }
    }

    pub fn map<NewItem, F: Fn(&Item) -> NewItem + 'a>(&self, map: F) -> Observable<'a, NewItem> {
        todo!();
    }
}
