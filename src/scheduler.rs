use std::ops::{Deref, DerefMut};

use crossbeam_queue::SegQueue;

use crate::emitter::Emitter;

pub trait Scheduler<'a>:
    'a + Deref<Target = Emitter<'a, Self::Event>> + DerefMut<Target = Emitter<'a, Self::Event>>
{
    type Event;

    fn on(emitter: Emitter<'a, Self::Event>) -> Self;

    fn publish(&mut self, event: Self::Event);

    fn finish(self) -> Emitter<'a, Self::Event>;
}

pub struct SingleThreadedScheduler<'a, Event>(Emitter<'a, Event>);

impl<'a, Event: 'a> Scheduler<'a> for SingleThreadedScheduler<'a, Event> {
    type Event = Event;

    fn on(emitter: Emitter<'a, Self::Event>) -> Self {
        Self(emitter)
    }

    fn publish(&mut self, event: Self::Event) {
        self.0.emit(&event)
    }

    fn finish(self) -> Emitter<'a, Self::Event> {
        self.0
    }
}

impl<'a, Event> Deref for SingleThreadedScheduler<'a, Event> {
    type Target = Emitter<'a, Event>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, Event> DerefMut for SingleThreadedScheduler<'a, Event> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct MultiThreadedScheduler<'a, Event> {
    emitter: Emitter<'a, Event>,
    event_queue: SegQueue<Event>,
}

impl<'a, Event: 'a> MultiThreadedScheduler<'a, Event> {
    pub fn emit_all(&mut self) {
        while !self.event_queue.is_empty() {
            self.emit_once();
        }
    }

    fn emit_once(&mut self) -> bool {
        self.event_queue
            .pop()
            .map(|event| self.emit_event(&event))
            .is_some()
    }

    fn emit_event(&mut self, event: &Event) {
        self.emitter.emit(event);
    }
}

impl<'a, Event: 'a> Scheduler<'a> for MultiThreadedScheduler<'a, Event> {
    type Event = Event;

    fn on(emitter: Emitter<'a, Self::Event>) -> Self {
        todo!(); // TODO: start a new thread that reads and publishes the events
        Self {
            emitter,
            event_queue: SegQueue::new(),
        }
    }

    fn publish(&mut self, event: Self::Event) {
        self.event_queue.push(event);
    }

    fn finish(mut self) -> Emitter<'a, Self::Event> {
        self.emit_all();

        self.emitter
    }
}

impl<'a, Event> Deref for MultiThreadedScheduler<'a, Event> {
    type Target = Emitter<'a, Event>;

    fn deref(&self) -> &Self::Target {
        &self.emitter
    }
}

impl<'a, Event> DerefMut for MultiThreadedScheduler<'a, Event> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.emitter
    }
}
