pub mod emitter;
pub mod observable;
pub mod scheduler;

#[cfg(test)]
mod tests {
    use crate::observable::Observable;

    #[test]
    fn test() {
        let mut total = 0;

        {
            let mut observable = Observable::new();

            observable.publish(5);
            observable.observe(|event| total += 3 * event);
            observable.publish(2);
        }

        assert_eq!(total, 6);
    }
}
