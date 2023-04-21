pub mod emitter;
pub mod observable;
pub mod scheduler;

#[cfg(test)]
mod tests {
    use crate::{
        observable::Observable,
        scheduler::{MultiThreadedScheduler, Scheduler},
    };

    #[test]
    fn test() {
        let mut total = 0;

        {
            let mut observable = Observable::new();

            observable.publish(5);
            observable.observe(|event| total += 3 * event);
            observable.publish(1);
            observable.observe(|event| println!("Got an event: {event}!"));
            observable.publish(2);
        }

        assert_eq!(total, 9);
    }

    #[test]
    fn test_scheduler() {
        let mut observable: Observable<&str, MultiThreadedScheduler<_>> =
            Observable::with_scheduler();

        observable.observe(|event| println!("woosh: {}", event));

        observable.publish("sus");
        observable.publish("eee");
        observable.publish("cool!");
    }
}
