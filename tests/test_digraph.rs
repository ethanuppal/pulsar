#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use pulsar_utils::digraph::Digraph;

    proptest! {
        #[test]
        fn test_add_node(nodes: Vec<u32>) {
            let mut graph: Digraph<u32, ()> = Digraph::new();
            for node in nodes.clone() {
                graph.add_node(node);
            }
            for node in nodes {
                prop_assert!(graph.out_of(node).is_some());
            }
        }

        #[test]
        fn test_add_edge(edges: Vec<(u32, u32, u32)>) {
            let mut graph = Digraph::new();
            for (u, e, v) in edges.clone() {
                graph.add_node(u);
                graph.add_node(v);
                graph.add_edge(u, e, v);
            }
            for (u, e, v) in edges {
                let out = graph.out_of(u).unwrap();
                prop_assert!(out.contains(&(e, v)));
            }
        }
    }
}
