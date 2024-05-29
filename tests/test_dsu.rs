#[cfg(test)]
mod tests {
    use proptest::{
        arbitrary::{any, Arbitrary},
        collection::hash_set,
        prelude::*
    };
    use pulsar::utils::{
        disjoint_set::{DisjointSets, NodeTrait},
        CheapClone
    };
    use std::fmt::Display;

    #[derive(Clone, Hash, PartialEq, Eq, Debug)]
    struct CheapCloneInt32 {
        value: i32
    }
    impl CheapClone for CheapCloneInt32 {}
    impl NodeTrait for CheapCloneInt32 {}
    impl Display for CheapCloneInt32 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.value.fmt(f)
        }
    }
    impl Arbitrary for CheapCloneInt32 {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            any::<i32>()
                .prop_map(|value| CheapCloneInt32 { value })
                .boxed()
        }
    }

    proptest! {
        #[test]
        fn test_find(values in hash_set(any::<CheapCloneInt32>(), 1..100)) {
            let mut ds = DisjointSets::new();
            for value in &values {
                ds.add(value.clone());
            }
            for value in &values {
                prop_assert_eq!(ds.find(value.clone()), Some(value.clone()));
            }
        }

        #[test]
        fn test_union(
            values1 in hash_set(any::<CheapCloneInt32>(), 1..50),
            values2 in hash_set(any::<CheapCloneInt32>(), 100..150),
            union_by_rank in bool::arbitrary()
        ) {
            let mut ds = DisjointSets::new();
            let mut iter1 = values1.iter();
            let mut iter2 = values2.iter();

            if let (Some(first1), Some(first2)) = (iter1.next(), iter2.next()) {
                ds.add(first1.clone());
                ds.add(first2.clone());

                for value in iter1 {
                    ds.add(value.clone());
                    ds.union(first1.clone(), value.clone(), union_by_rank);
                }

                for value in iter2 {
                    ds.add(value.clone());
                    ds.union(first2.clone(), value.clone(), union_by_rank);
                }

                ds.union(first1.clone(), first2.clone(), union_by_rank);

                for value in values1.iter().chain(values2.iter()) {
                    prop_assert_eq!(ds.find(first1.clone()), ds.find(value.clone()));
                }
            }
        }
    }
}
