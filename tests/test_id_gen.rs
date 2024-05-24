#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use pulsar::utils::id::Gen;

    proptest! {
        #[test]
        fn test_next_uniqueness(loop_count in 0..1000) {
            let mut ids = std::collections::HashSet::new();
            for _ in 0..loop_count {
                let id = Gen::next("test");
                assert!(ids.insert(id));
            }
        }
    }
}
