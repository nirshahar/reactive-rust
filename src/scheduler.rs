use std::ops::Deref;

use crate::observable::Observable;

trait Scheduler<'a>: 'a + Deref<Target = Observable<'a, Self::Event>> {
    type Event;

    fn on(observable: &'a Observable<'a, Self::Event>) -> Self;

    fn publish(&self, event: Self::Event);
}

pub struct SingleThreadedScheduler<'a, Event>(&'a Observable<'a, Event>);

impl<'a, Event> Scheduler<'a> for SingleThreadedScheduler<'a, Event> {
    type Event = Event;

    fn on(observable: &'a Observable<'a, Self::Event>) -> Self {
        Self(observable)
    }

    fn publish(&self, event: Self::Event) {
        self.emit_event(&event)
    }
}

impl<'a, Event> Deref for SingleThreadedScheduler<'a, Event> {
    type Target = Observable<'a, Event>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
