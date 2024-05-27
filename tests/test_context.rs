#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use pulsar::utils::environment::Environment;

    proptest! {
        #[test]
        fn test_bind_and_find(name in "\\PC*", value: i32) {
            let mut context = Environment::new();
            context.bind(name.clone(), value);
            prop_assert_eq!(context.find(name), Some(&value));
        }

        #[test]
        fn test_push_and_pop(name in "\\PC*", value: i32) {
            let mut context = Environment::new();
            context.push();
            context.bind(name.clone(), value);
            prop_assert!(context.find(name.clone()).is_some());
            prop_assert!(context.pop());
            prop_assert!(context.find(name).is_none());
        }
    }
}
