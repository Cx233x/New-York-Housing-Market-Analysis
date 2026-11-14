use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    address: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(graph: &HashMap<String, HashMap<String, u64>>, start: &str, goal: &str) -> Option<u64> {
    let mut distances: HashMap<String, u64> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start.to_string(), 0);
    heap.push(State { cost: 0, address: start.to_string() });

    while let Some(State { cost, address }) = heap.pop() {
        if address == goal {
            return Some(cost);
        }

        if let Some(neighbors) = graph.get(&address) {
            for (neighbor, &price) in neighbors {
                let next = State { cost: cost + price, address: neighbor.clone() };
                let current_shortest = distances.get(neighbor).copied().unwrap_or(u64::MAX);

                if next.cost < current_shortest {
                    heap.push(next.clone());
                    distances.insert(neighbor.clone(), next.cost);
                }
            }
        }
    }

    None
}
