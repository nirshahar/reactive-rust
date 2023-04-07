pub struct Observable<'a, Item> {
    observers: Vec<Box<dyn Fn(&Item) + 'a>>,
}

impl<'a, Item> Observable<'a, Item> {
    pub fn new() -> Self {
        return Self {
            observers: Vec::new(),
        };
    }

    pub fn observe<F: Fn(&Item) + 'a>(&mut self, observer: F) {
        self.observers.push(Box::new(observer));
    }

    pub fn emit(&self, item: &Item) {
        for obs in self.observers.iter() {
            obs(item);
        }
    }

    pub fn map<NewItem, F: Fn(&Item) -> NewItem + 'a>(&self, map: F) -> Observable<'a, NewItem> {
        todo!();
    }
}
