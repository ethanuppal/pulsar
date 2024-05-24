#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use pulsar::utils::id::Gen;

    proptest! {
        #[test]
        fn test_next_uniqueness(name in any::<String>()) {
            let mut ids = std::collections::HashSet::new();
            for _ in 0..1000 {
                let id = Gen::next(name.clone());
                assert!(ids.insert(id));
            }
        }
    }
}
