//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use pulsar_utils::id::Gen;

    proptest! {
        #[test]
        fn test_next_uniqueness(loop_count in 0..1000) {
            let mut gen = Gen::new();
            let mut ids = std::collections::HashSet::new();
            for _ in 0..loop_count {
                assert!(ids.insert(gen.next()));
            }
        }
    }
}
