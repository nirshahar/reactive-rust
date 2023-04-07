use crossbeam_queue::SegQueue;

pub struct Observable<'a, Item> {
    observers: Vec<Box<dyn Fn(&Item) + 'a>>,
    event_queue: SegQueue<&'a Item>,
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

    pub fn publish(&self, item: &'a Item) {
        self.event_queue.push(item);
    }

    pub fn emit_once(&self) -> bool {
        if let Some(item) = self.event_queue.pop() {
            self.emit_item(item);

            true
        } else {
            false
        }
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
