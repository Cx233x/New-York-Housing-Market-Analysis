mod parser;
mod calculation;
mod graph;

use parser::parse_csv;
use calculation::{
    degree_distribution, 
    mean, 
    variance, 
    standard_deviation, 
    mode, 
    edge_weight_analysis, 
    price_distribution, 
    pricing_differences
};
use graph::dijkstra;
use std::collections::HashMap;

fn main() {
    let file_path = concat!(
        env!("CARGO_MANIFEST_DIR"), 
        "/src/NY-House-Dataset.csv"
    );
    let graph = parse_csv(file_path)
        .expect("Failed to parse CSV file");

    // Degree Centrality
    let degrees = degree_distribution(&graph);
    println!("Degree Distribution: {:?}", degrees);
    let mean_degree = mean(&degrees);
    println!("Mean degree: {}", mean_degree);
    println!("Variance: {}", variance(&degrees, mean_degree));
    println!("Standard Deviation: {}", standard_deviation(&degrees, mean_degree));
    println!("Mode: {:?}", mode(&degrees));

    // Edge Weight Analysis
    let edge_analysis = edge_weight_analysis(&graph);
    println!("Edge Weight Analysis: {:?}", edge_analysis);

    // Shortest Path Analysis
    let node1 = "Condo";
    let node2 = "Co-op";
    if let Some(distance) = dijkstra(&graph, node1, node2) {
        println!("Shortest Path between {} and {} is {}", node1, node2, distance);
    } else {
        println!("No path between {} and {}", node1, node2);
    }

    // Price Distribution
    let distributions = price_distribution(&graph);
    println!("Price Distributions: {:?}", distributions);
    let pricing_diff = pricing_differences(&distributions);
    println!("Price Differences: {:?}", pricing_diff);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_degree_distribution() {
        let mut graph = HashMap::new();
        graph.insert("Node1".to_string(), HashMap::from([("Node2".to_string(), 10)]));
        graph.insert("Node2".to_string(), HashMap::from([("Node3".to_string(), 20)]));
        let degrees = degree_distribution(&graph);
        assert_eq!(degrees, HashMap::from([("Node1".to_string(), 1), ("Node2".to_string(), 1)]));
    }

    #[test]
    fn test_edge_weight_analysis() {
        let graph = HashMap::from([("Node1".to_string(), HashMap::from([("Node2".to_string(), 10), ("Node3".to_string(), 20)]))]);
        let analysis = edge_weight_analysis(&graph);
        assert_eq!(analysis, HashMap::from([(("Node1".to_string(), "Node2".to_string()), (10.0, 10, 10)), (("Node1".to_string(), "Node3".to_string()), (20.0, 20, 20))]));
    }

    #[test]
    fn test_shortest_path() {
    let graph = HashMap::from([("Node1".to_string(), HashMap::from([("Node2".to_string(), 10)]))]);
    let distance = dijkstra(&graph, "Node1", "Node3");
    assert_eq!(distance, None); 

    let distance = dijkstra(&graph, "Node1", "Node2");
    assert_eq!(distance, Some(10));  

    let graph = HashMap::from([
        ("Node1".to_string(), HashMap::from([("Node2".to_string(), 5), ("Node3".to_string(), 2)])),
        ("Node2".to_string(), HashMap::from([("Node3".to_string(), 1)])),
        ("Node3".to_string(), HashMap::from([("Node4".to_string(), 3)])),
        ("Node4".to_string(), HashMap::new())
    ]);
    let distance = dijkstra(&graph, "Node1", "Node4");
    assert_eq!(distance, Some(5));  // Shortest path from Node1 to Node4 is through Node3
}

}
