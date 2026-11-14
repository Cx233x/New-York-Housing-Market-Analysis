use crate::graph::Graph;
use csv::{ReaderBuilder, Trim};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Property {
    #[serde(rename = "TYPE")]
    property_type: String,
    #[serde(rename = "PRICE")]
    price: Option<u64>,
}

pub fn parse_csv(file_path: &Path) -> Result<Graph, Box<dyn Error>> {
    build_graph(File::open(file_path)?)
}

fn build_graph(reader: impl Read) -> Result<Graph, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(reader);
    for required in ["TYPE", "PRICE"] {
        if !rdr.headers()?.iter().any(|header| header == required) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("missing CSV column: {required}"),
            )
            .into());
        }
    }
    let mut ranges: HashMap<String, (u64, u64)> = HashMap::new();
    for result in rdr.deserialize() {
        let record: Property = result?;
        if record.property_type.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "empty TYPE in CSV").into());
        }
        if let Some(price) = record.price {
            let range = ranges.entry(record.property_type).or_insert((price, price));
            range.0 = range.0.min(price);
            range.1 = range.1.max(price);
        }
    }

    // Keep categories with valid prices even when they have no overlapping neighbors.
    let mut graph: Graph = ranges
        .keys()
        .map(|name| (name.clone(), HashMap::new()))
        .collect();
    for (a, &(min_a, max_a)) in &ranges {
        for (b, &(min_b, max_b)) in &ranges {
            if a != b && max_a >= min_b && max_b >= min_a {
                let overlap = max_a.min(max_b) - min_a.max(min_b);
                graph
                    .get_mut(a)
                    .expect("category initialized")
                    .insert(b.clone(), overlap);
            }
        }
    }
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_touching_intervals_missing_prices_and_isolated_nodes() {
        let csv = "TYPE,PRICE\nA,10\nA,20\nB,15\nB,25\nC,25\nD,100\nMissing,\n";
        let graph = build_graph(csv.as_bytes()).unwrap();
        assert_eq!(graph.len(), 4);
        assert_eq!(graph["A"]["B"], 5);
        assert_eq!(graph["B"]["A"], 5);
        assert_eq!(graph["B"]["C"], 0);
        assert!(graph["D"].is_empty());
        assert!(!graph["A"].contains_key("A"));
    }

    #[test]
    fn invalid_schema_categories_and_prices_are_errors() {
        for csv in [
            "TYPE\nA\n",
            "TYPE,PRICE\n,10\n",
            "TYPE,PRICE\nA,-1\n",
            "TYPE,PRICE\nA,text\n",
        ] {
            assert!(build_graph(csv.as_bytes()).is_err());
        }
        assert!(build_graph("TYPE,PRICE\n".as_bytes()).unwrap().is_empty());
    }

    #[test]
    fn bundled_dataset_matches_documented_graph() {
        let graph = build_graph(include_bytes!("NY-House-Dataset.csv").as_slice()).unwrap();
        assert_eq!(graph.len(), 13);
        assert_eq!(graph.values().map(HashMap::len).sum::<usize>(), 154);
        assert_eq!(graph["Townhouse for sale"]["House for sale"], 64_685_000);
        assert_eq!(graph["Coming Soon"]["Foreclosure"], 1_046_000);
        let names: Vec<_> = graph.keys().collect();
        for a in &names {
            for b in &names {
                let heap = crate::graph::dijkstra_heap(&graph, a, b).unwrap();
                let scan = crate::graph::dijkstra_linear_scan(&graph, a, b).unwrap();
                assert_eq!(heap.as_ref().map(|p| p.cost), scan.as_ref().map(|p| p.cost));
            }
        }
        let path = crate::graph::dijkstra_heap(&graph, "Condo for sale", "Co-op for sale")
            .unwrap()
            .unwrap();
        assert_eq!(path.cost, 0);
        assert_eq!(
            path.nodes,
            vec!["Condo for sale", "Mobile house for sale", "Co-op for sale"]
        );
    }
}
