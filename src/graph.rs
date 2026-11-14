use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fmt;

pub type Graph = HashMap<String, HashMap<String, u64>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub cost: u64,
    pub nodes: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PathError {
    UnknownNode(String),
    CostOverflow,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownNode(node) => write!(f, "unknown category: {node}; use --list-types"),
            Self::CostOverflow => write!(f, "an examined path cost exceeds u64::MAX"),
        }
    }
}

impl Error for PathError {}

fn validate_nodes(graph: &Graph, start: &str, goal: &str) -> Result<(), PathError> {
    for node in [start, goal] {
        if !graph.contains_key(node) {
            return Err(PathError::UnknownNode(node.to_owned()));
        }
    }
    Ok(())
}

fn reconstruct(cost: u64, goal: &str, parents: &HashMap<String, String>) -> Path {
    let mut nodes = vec![goal.to_owned()];
    while let Some(parent) = parents.get(nodes.last().expect("path has a goal")) {
        nodes.push(parent.clone());
    }
    nodes.reverse();
    Path { cost, nodes }
}

/// Dijkstra with a binary heap. Equal costs are ordered by category name.
pub fn dijkstra_heap(graph: &Graph, start: &str, goal: &str) -> Result<Option<Path>, PathError> {
    validate_nodes(graph, start, goal)?;
    let mut distances = HashMap::from([(start.to_owned(), 0)]);
    let mut parents = HashMap::new();
    let mut heap = BinaryHeap::from([Reverse((0_u64, start.to_owned()))]);

    while let Some(Reverse((cost, node))) = heap.pop() {
        if distances.get(&node) != Some(&cost) {
            continue;
        }
        if node == goal {
            return Ok(Some(reconstruct(cost, goal, &parents)));
        }
        if let Some(neighbors) = graph.get(&node) {
            for (neighbor, weight) in neighbors {
                let next = cost.checked_add(*weight).ok_or(PathError::CostOverflow)?;
                if distances.get(neighbor).map_or(true, |&old| next < old) {
                    distances.insert(neighbor.clone(), next);
                    parents.insert(neighbor.clone(), node.clone());
                    heap.push(Reverse((next, neighbor.clone())));
                }
            }
        }
    }
    Ok(None)
}

/// Dijkstra with a linear scan of the frontier instead of a priority queue.
pub fn dijkstra_linear_scan(
    graph: &Graph,
    start: &str,
    goal: &str,
) -> Result<Option<Path>, PathError> {
    validate_nodes(graph, start, goal)?;
    let mut distances = HashMap::from([(start.to_owned(), 0_u64)]);
    let mut parents = HashMap::new();
    let mut visited = HashSet::new();
    let mut frontier = HashSet::from([start.to_owned()]);

    while let Some(node) = frontier
        .iter()
        .min_by_key(|node| (distances[*node], *node))
        .cloned()
    {
        frontier.remove(&node);
        let cost = distances[&node];
        if node == goal {
            return Ok(Some(reconstruct(cost, goal, &parents)));
        }
        visited.insert(node.clone());
        if let Some(neighbors) = graph.get(&node) {
            for (neighbor, weight) in neighbors {
                // Check all examined sums, matching the heap implementation's policy.
                let next = cost.checked_add(*weight).ok_or(PathError::CostOverflow)?;
                if !visited.contains(neighbor)
                    && distances.get(neighbor).map_or(true, |&old| next < old)
                {
                    distances.insert(neighbor.clone(), next);
                    parents.insert(neighbor.clone(), node.clone());
                    frontier.insert(neighbor.clone());
                }
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Search = fn(&Graph, &str, &str) -> Result<Option<Path>, PathError>;
    const SEARCHES: [Search; 2] = [dijkstra_heap, dijkstra_linear_scan];

    fn fixture() -> Graph {
        HashMap::from([
            (
                "A".into(),
                HashMap::from([("B".into(), 8), ("C".into(), 2)]),
            ),
            ("B".into(), HashMap::from([("D".into(), 1)])),
            (
                "C".into(),
                HashMap::from([("B".into(), 1), ("D".into(), 9)]),
            ),
            ("D".into(), HashMap::new()),
            ("isolated".into(), HashMap::new()),
        ])
    }

    #[test]
    fn cheaper_indirect_route_and_unreachable_nodes() {
        for search in SEARCHES {
            let graph = fixture();
            assert_eq!(
                search(&graph, "A", "D").unwrap(),
                Some(Path {
                    cost: 4,
                    nodes: vec!["A".into(), "C".into(), "B".into(), "D".into()],
                })
            );
            assert_eq!(search(&graph, "A", "isolated").unwrap(), None);
            assert_eq!(search(&graph, "A", "A").unwrap().unwrap().cost, 0);
            assert_eq!(
                search(&graph, "missing", "missing"),
                Err(PathError::UnknownNode("missing".into()))
            );
            assert!(matches!(
                search(&graph, "A", "missing"),
                Err(PathError::UnknownNode(_))
            ));
        }
    }

    #[test]
    fn zero_weight_cycles_and_equal_cost_routes() {
        let graph = HashMap::from([
            (
                "A".into(),
                HashMap::from([("B".into(), 0), ("C".into(), 0)]),
            ),
            (
                "B".into(),
                HashMap::from([("A".into(), 0), ("D".into(), 1)]),
            ),
            ("C".into(), HashMap::from([("D".into(), 1)])),
            ("D".into(), HashMap::new()),
        ]);
        for search in SEARCHES {
            let path = search(&graph, "A", "D").unwrap().unwrap();
            assert_eq!(path.cost, 1);
            assert_eq!(path.nodes, vec!["A", "B", "D"]);
        }
    }

    #[test]
    fn maximum_distance_is_valid_but_overflow_is_an_error() {
        let mut graph = HashMap::from([
            ("A".into(), HashMap::from([("B".into(), u64::MAX)])),
            ("B".into(), HashMap::from([("C".into(), 1)])),
            ("C".into(), HashMap::new()),
        ]);
        for search in SEARCHES {
            assert_eq!(search(&graph, "A", "B").unwrap().unwrap().cost, u64::MAX);
            assert_eq!(search(&graph, "A", "C"), Err(PathError::CostOverflow));
        }
        graph.get_mut("B").unwrap().insert("C".into(), 0);
        for search in SEARCHES {
            assert_eq!(search(&graph, "A", "C").unwrap().unwrap().cost, u64::MAX);
        }
    }

    /// An independent all-pairs dynamic-programming oracle, with a fixed seed.
    #[test]
    fn both_searches_match_floyd_warshall() {
        let mut seed = 42_u64;
        for size in [2, 5, 12] {
            let names: Vec<String> = (0..size).map(|n| format!("N{n:02}")).collect();
            let mut graph: Graph = names.iter().map(|n| (n.clone(), HashMap::new())).collect();
            let mut expected = vec![vec![None; size]; size];
            for a in 0..size {
                expected[a][a] = Some(0_u64);
                for b in 0..size {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    if a != b && seed % 3 != 0 {
                        let weight = (seed >> 32) % 20;
                        graph
                            .get_mut(&names[a])
                            .unwrap()
                            .insert(names[b].clone(), weight);
                        expected[a][b] = Some(weight);
                    }
                }
            }
            for k in 0..size {
                for a in 0..size {
                    for b in 0..size {
                        if let (Some(left), Some(right)) = (expected[a][k], expected[k][b]) {
                            let cost = left + right;
                            if expected[a][b].map_or(true, |old| cost < old) {
                                expected[a][b] = Some(cost);
                            }
                        }
                    }
                }
            }
            for a in 0..size {
                for b in 0..size {
                    for search in SEARCHES {
                        let result = search(&graph, &names[a], &names[b]).unwrap();
                        assert_eq!(result.as_ref().map(|p| p.cost), expected[a][b]);
                        if let Some(path) = result {
                            assert_eq!(path.nodes.first(), Some(&names[a]));
                            assert_eq!(path.nodes.last(), Some(&names[b]));
                            let sum: u64 = path
                                .nodes
                                .windows(2)
                                .map(|pair| graph[&pair[0]][&pair[1]])
                                .sum();
                            assert_eq!(sum, path.cost);
                        }
                    }
                }
            }
        }
    }
}
