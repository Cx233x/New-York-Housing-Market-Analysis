use crate::graph::Graph;
use std::collections::HashMap;

pub fn degree_distribution(graph: &Graph) -> HashMap<String, usize> {
    graph.iter().map(|(k, v)| (k.clone(), v.len())).collect()
}

pub fn mean(degrees: &HashMap<String, usize>) -> Option<f64> {
    if degrees.is_empty() {
        return None;
    }
    Some(degrees.values().sum::<usize>() as f64 / degrees.len() as f64)
}

pub fn variance(degrees: &HashMap<String, usize>, mean: f64) -> Option<f64> {
    if degrees.is_empty() {
        return None;
    }
    Some(
        degrees
            .values()
            .map(|&deg| (deg as f64 - mean).powi(2))
            .sum::<f64>()
            / degrees.len() as f64,
    )
}

pub fn mode(degrees: &HashMap<String, usize>) -> Vec<usize> {
    let mut frequency = HashMap::new();
    for degree in degrees.values() {
        *frequency.entry(*degree).or_insert(0) += 1;
    }
    let max = frequency.values().copied().max().unwrap_or(0);
    let mut modes: Vec<_> = frequency
        .into_iter()
        .filter(|(_, count)| *count == max)
        .map(|(degree, _)| degree)
        .collect();
    modes.sort_unstable();
    modes
}

/// The parser builds an undirected graph; include each edge exactly once.
pub fn unique_edges(graph: &Graph) -> Vec<(&str, &str, u64)> {
    let mut edges: Vec<_> = graph
        .iter()
        .flat_map(|(a, neighbors)| {
            neighbors
                .iter()
                .filter(move |(b, _)| a < *b)
                .map(move |(b, &weight)| (a.as_str(), b.as_str(), weight))
        })
        .collect();
    edges.sort_unstable();
    edges
}

#[derive(Debug, PartialEq)]
pub struct WeightSummary {
    pub min: u64,
    pub median: f64,
    pub max: u64,
    pub mean: f64,
}

pub fn weight_summary(weights: impl IntoIterator<Item = u64>) -> Option<WeightSummary> {
    let mut weights: Vec<_> = weights.into_iter().collect();
    weights.sort_unstable();
    let n = weights.len();
    if n == 0 {
        return None;
    }
    let median = if n % 2 == 0 {
        weights[n / 2 - 1] as f64 / 2.0 + weights[n / 2] as f64 / 2.0
    } else {
        weights[n / 2] as f64
    };
    Some(WeightSummary {
        min: weights[0],
        median,
        max: weights[n - 1],
        mean: weights.iter().map(|&w| w as f64).sum::<f64>() / n as f64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undirected_edges_are_counted_once_and_isolates_are_retained() {
        let graph = HashMap::from([
            ("A".into(), HashMap::from([("B".into(), 10)])),
            ("B".into(), HashMap::from([("A".into(), 10)])),
            ("C".into(), HashMap::new()),
        ]);
        let degrees = degree_distribution(&graph);
        assert_eq!(degrees["C"], 0);
        assert_eq!(mode(&degrees), vec![1]);
        assert_eq!(unique_edges(&graph), vec![("A", "B", 10)]);
        let average = mean(&degrees).unwrap();
        assert!((average - 2.0 / 3.0).abs() < 1e-10);
        assert!((variance(&degrees, average).unwrap() - 2.0 / 9.0).abs() < 1e-10);
    }

    #[test]
    fn empty_statistics_and_fractional_median() {
        assert_eq!(mean(&HashMap::new()), None);
        assert_eq!(variance(&HashMap::new(), 0.0), None);
        assert!(mode(&HashMap::new()).is_empty());
        assert_eq!(weight_summary([]), None);
        assert_eq!(
            weight_summary([2, 1]),
            Some(WeightSummary {
                min: 1,
                median: 1.5,
                max: 2,
                mean: 1.5
            })
        );
    }
}
