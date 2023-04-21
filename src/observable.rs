use std::marker::PhantomData;

use crate::{
    emitter::{Emitter, ObservationKey},
    scheduler::{Scheduler, SingleThreadedScheduler},
};

pub struct Observable<'a, Event, Schedule>
where
    Schedule: Scheduler<'a, Event = Event>,
{
    scheduler: Schedule,
    _phantom_event: PhantomData<&'a Event>,
}

impl<'a, Event, Schedule> Observable<'a, Event, Schedule>
where
    Schedule: Scheduler<'a, Event = Event>,
{
    pub fn observe<F: FnMut(&Event) + 'a>(&mut self, subscriber: F) -> ObservationKey {
        self.scheduler.observe(subscriber)
    }

    pub fn publish(&mut self, event: Event) {
        self.scheduler.publish(event);
    }

    pub fn with_scheduler() -> Observable<'a, Event, Schedule> {
        let observable = Observable::new();

        let emmiter = observable.scheduler.finish();

        let scheduler = Schedule::on(emmiter);

        Observable {
            scheduler,
            _phantom_event: PhantomData,
        }
    }
}

impl<'a, Event> Observable<'a, Event, SingleThreadedScheduler<'a, Event>> {
    pub fn new() -> Self {
        let emmiter = Emitter::new();
        Self {
            scheduler: SingleThreadedScheduler::on(emmiter),
            _phantom_event: PhantomData,
        }
    }
}
