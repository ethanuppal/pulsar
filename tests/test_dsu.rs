//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[cfg(test)]
mod tests {
    use proptest::{
        arbitrary::{any, Arbitrary},
        collection::hash_set,
        prelude::*
    };
    use pulsar_utils::disjoint_sets::DisjointSets;

    proptest! {
        #[test]
        fn test_find(values in hash_set(any::<i32>(), 1..100)) {
            let mut ds = DisjointSets::new();
            for value in &values {
                ds.add(value);
            }
            for value in &values {
                prop_assert_eq!(ds.find(value), Some(value));
            }
        }

        #[test]
        fn test_union(
            values1 in hash_set(any::<i32>(), 1..50),
            values2 in hash_set(any::<i32>(), 100..150),
            union_by_rank in bool::arbitrary()
        ) {
            let mut ds = DisjointSets::new();
            let mut iter1 = values1.iter();
            let mut iter2 = values2.iter();

            if let (Some(first1), Some(first2)) = (iter1.next(), iter2.next()) {
                ds.add(first1);
                ds.add(first2);

                for value in iter1 {
                    ds.add(value);
                    ds.union(first1, value, union_by_rank);
                }

                for value in iter2 {
                    ds.add(value);
                    ds.union(first2, value, union_by_rank);
                }

                ds.union(first1, first2, union_by_rank);

                for value in values1.iter().chain(values2.iter()) {
                    prop_assert_eq!(ds.find(first1), ds.find(value));
                }
            }
        }
    }
}
