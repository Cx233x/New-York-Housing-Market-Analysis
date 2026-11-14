use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::borrow::Borrow;

pub fn degree_distribution(graph: &HashMap<String, HashMap<String, u64>>) -> HashMap<String, usize> {
    graph.iter().map(|(k, v)| (k.clone(), v.len())).collect()
}

pub fn mean(degrees: &HashMap<String, usize>) -> f64 {
    let sum: usize = degrees.values().sum();
    sum as f64 / degrees.len() as f64
}

pub fn variance(degrees: &HashMap<String, usize>, mean: f64) -> f64 {
    degrees.values().map(|&deg| {
        let diff = (deg as f64) - mean;
        diff * diff
    }).sum::<f64>() / degrees.len() as f64
}

pub fn standard_deviation(degrees: &HashMap<String, usize>, mean: f64) -> f64 {
    variance(degrees, mean).sqrt()
}

pub fn mode(degrees: &HashMap<String, usize>) -> Vec<usize> {
    let mut frequency = HashMap::new();
    for degree in degrees.values() {
        *frequency.entry(*degree).or_insert(0) += 1;
    }
    let max_freq = frequency.values().max().unwrap_or(&0);
    frequency.iter().filter(|&(_, &v)| v == *max_freq).map(|(&k, _)| k).collect()
}

fn u64_cmp(a: &u64, b: &u64) -> Ordering {
    a.cmp(b)
}

pub fn edge_weight_analysis(graph: &HashMap<String, HashMap<String, u64>>) -> HashMap<(String, String), (f64, u64, u64)> {
    let mut result = HashMap::new();

    for (node1, edges) in graph.iter() {
        for (node2, &weight) in edges.iter() {
            let entry = result.entry((node1.clone(), node2.clone())).or_insert_with(Vec::new);
            entry.push(weight);
        }
    }

    result.into_iter().map(|(key, values)| {
        let avg = values.iter().cloned().sum::<u64>() as f64 / values.len() as f64;
        let max = values.iter().cloned().fold(u64::MIN, u64::max);
        let median = {
            let sorted = values.iter().cloned().sorted_by(u64_cmp).collect::<Vec<_>>();
            if sorted.len() % 2 == 0 {
                (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2
            } else {
                sorted[sorted.len() / 2]
            }
        };
        (key, (avg, median, max))
    }).collect()
}

pub fn shortest_path(graph: &HashMap<String, HashMap<String, u64>>, start: &str, end: &str) -> Option<u64> {
    use std::cmp::min;

    let mut distances: HashMap<String, u64> = graph.keys().map(|k| (k.clone(), u64::MAX)).collect();
    let mut visited = HashSet::new();
    let mut to_visit = HashSet::new();
    distances.insert(start.to_string(), 0);
    to_visit.insert(start.to_string());

    while let Some(current) = to_visit.iter().min_by_key(|&&ref a| distances[a]).cloned() {
        to_visit.remove(&current);
        visited.insert(current.clone());

        if &current == end {
            return Some(distances[&current]);
        }

        if let Some(neighbors) = graph.get(&current) {
            for (neighbor, &weight) in neighbors {
                if !visited.contains(neighbor) {
                    let tentative_distance = distances[&current].saturating_add(weight);
                    distances.entry(neighbor.clone()).and_modify(|d| *d = min(*d, tentative_distance)).or_insert(tentative_distance);
                    to_visit.insert(neighbor.clone());
                }
            }
        }
    }

    None
}


pub fn price_distribution(graph: &HashMap<String, HashMap<String, u64>>) -> HashMap<String, Vec<u64>> {
    let mut distributions = HashMap::new();

    for (node, edges) in graph.iter() {
        for &price in edges.values() {
            distributions.entry(node.clone()).or_insert_with(Vec::new).push(price);
        }
    }

    distributions
}

pub fn pricing_differences(prices: &HashMap<String, Vec<u64>>) -> HashMap<String, (u64, u64, f64)> {
    prices.iter().map(|(k, v)| {
        let min = v.iter().cloned().fold(u64::MAX, u64::min);
        let max = v.iter().cloned().fold(u64::MIN, u64::max);
        let mean = v.iter().cloned().sum::<u64>() as f64 / v.len() as f64;
        (k.clone(), (min, max, mean))
    }). collect()
}
