mod calculation;
mod graph;
mod parser;

use calculation::{degree_distribution, mean, mode, unique_edges, variance, weight_summary};
use graph::{dijkstra_heap, dijkstra_linear_scan, Graph, Path, PathError};
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

const HELP: &str = "New York housing price-range graph analysis

Usage: ny-housing-graph [OPTIONS]

Options:
  --data PATH        CSV file (default: bundled data/NY-House-Dataset.csv; download separately)
  --start TYPE       Start category (default: Condo for sale)
  --end TYPE         Destination category (default: Co-op for sale)
  --algorithm NAME   heap, scan, or both (default: both)
  --repeat N         Positive number of timed queries per algorithm (default: 1)
  --list-types       List available categories and exit
  -h, --help         Print help

Example:
  cargo run --release -- --data data/NY-House-Dataset.csv --list-types

Costs are cumulative price-range overlap widths, not travel distances or similarity scores.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Heap,
    Scan,
    Both,
}

#[derive(Debug)]
struct Options {
    data: PathBuf,
    start: String,
    end: String,
    algorithm: Algorithm,
    repeat: u32,
    list_types: bool,
}

impl Options {
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Option<Self>, String> {
        let mut options = Self {
            data: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/NY-House-Dataset.csv"),
            start: "Condo for sale".into(),
            end: "Co-op for sale".into(),
            algorithm: Algorithm::Both,
            repeat: 1,
            list_types: false,
        };
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => return Ok(None),
                "--list-types" => options.list_types = true,
                "--data" | "--start" | "--end" | "--algorithm" | "--repeat" => {
                    let value = args
                        .next()
                        .filter(|value| !value.starts_with("--"))
                        .ok_or_else(|| format!("{arg} requires a value"))?;
                    match arg.as_str() {
                        "--data" => options.data = value.into(),
                        "--start" => options.start = value,
                        "--end" => options.end = value,
                        "--algorithm" => {
                            options.algorithm = match value.as_str() {
                                "heap" => Algorithm::Heap,
                                "scan" => Algorithm::Scan,
                                "both" => Algorithm::Both,
                                _ => return Err("--algorithm must be heap, scan, or both".into()),
                            }
                        }
                        "--repeat" => {
                            options.repeat = value
                                .parse::<u32>()
                                .ok()
                                .filter(|n| *n > 0)
                                .ok_or("--repeat must be an integer from 1 to 4294967295")?;
                        }
                        _ => unreachable!(),
                    }
                }
                _ => return Err(format!("unknown option: {arg}; use --help")),
            }
        }
        Ok(Some(options))
    }
}

type Search = fn(&Graph, &str, &str) -> Result<Option<Path>, PathError>;

fn timed_search(
    search: Search,
    graph: &Graph,
    options: &Options,
) -> Result<(Option<Path>, Duration), PathError> {
    // Warm up outside the measured interval; errors are reported before printing timings.
    let mut result = search(graph, &options.start, &options.end)?;
    let started = Instant::now();
    for _ in 0..options.repeat {
        result = std::hint::black_box(search(
            std::hint::black_box(graph),
            std::hint::black_box(&options.start),
            std::hint::black_box(&options.end),
        )?);
    }
    Ok((result, started.elapsed()))
}

fn print_statistics(graph: &Graph) {
    let degrees = degree_distribution(graph);
    let mut categories: Vec<_> = graph.keys().collect();
    categories.sort();
    println!("Category connectivity and incident overlap widths (USD)");
    println!(
        "{:<28} {:>6} {:>14} {:>14} {:>14}",
        "Category", "Degree", "Min overlap", "Mean overlap", "Max overlap"
    );
    for name in categories {
        if let Some(summary) = weight_summary(graph[name].values().copied()) {
            println!(
                "{:<28} {:>6} {:>14} {:>14.2} {:>14}",
                name, degrees[name], summary.min, summary.mean, summary.max
            );
        } else {
            println!(
                "{:<28} {:>6} {:>14} {:>14} {:>14}",
                name, degrees[name], "N/A", "N/A", "N/A"
            );
        }
    }
    if let Some(mean_degree) = mean(&degrees) {
        let variance = variance(&degrees, mean_degree).expect("nonempty degrees");
        let modes = mode(&degrees)
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        println!("\nDegree mean: {mean_degree:.6}; population variance: {variance:.6}; standard deviation: {:.6}; mode: {modes}", variance.sqrt());
    }
    let edges = unique_edges(graph);
    println!(
        "Categories: {}; undirected edges: {}",
        graph.len(),
        edges.len()
    );
    if let Some(summary) = weight_summary(edges.iter().map(|edge| edge.2)) {
        println!(
            "Unique-edge overlap widths (USD): min={}, median={:.2}, mean={:.2}, max={}",
            summary.min, summary.median, summary.mean, summary.max
        );
    }
    println!(
        "\n{:<28} {:<28} {:>14}",
        "Category A", "Category B", "Overlap (USD)"
    );
    for (a, b, weight) in edges {
        println!("{a:<28} {b:<28} {weight:>14}");
    }
}

fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let graph = parser::parse_csv(&options.data)
        .map_err(|error| format!("cannot load {}: {error}", options.data.display()))?;
    if graph.is_empty() {
        return Err("no categories with valid prices were found".into());
    }
    if options.list_types {
        let mut names: Vec<_> = graph.keys().collect();
        names.sort();
        for name in names {
            println!("{name}");
        }
        return Ok(());
    }
    let searches: &[(&str, Search)] = match options.algorithm {
        Algorithm::Heap => &[("heap", dijkstra_heap)],
        Algorithm::Scan => &[("scan", dijkstra_linear_scan)],
        Algorithm::Both => &[("heap", dijkstra_heap), ("scan", dijkstra_linear_scan)],
    };
    let mut results = Vec::new();
    for &(name, search) in searches {
        let (path, elapsed) = timed_search(search, &graph, &options)?;
        results.push((name, path, elapsed));
    }
    if results.len() == 2
        && results[0].1.as_ref().map(|p| p.cost) != results[1].1.as_ref().map(|p| p.cost)
    {
        return Err("algorithm disagreement: reachability or minimum costs differ".into());
    }
    print_statistics(&graph);
    println!(
        "\nMinimum cumulative overlap: {} -> {}",
        options.start, options.end
    );
    println!(
        "{:<12} {:>16} {:>12} {:>18}",
        "Algorithm", "Cost (USD)", "Queries", "Mean query (ns)"
    );
    for (name, path, elapsed) in &results {
        let cost = path
            .as_ref()
            .map(|p| p.cost.to_string())
            .unwrap_or_else(|| "unreachable".into());
        println!(
            "{:<12} {:>16} {:>12} {:>18.2}",
            name,
            cost,
            options.repeat,
            elapsed.as_nanos() as f64 / f64::from(options.repeat)
        );
    }
    for (name, path, _) in &results {
        if let Some(path) = path {
            println!("{name} route: {}", path.nodes.join(" -> "));
        }
    }
    if results.len() == 2 {
        println!("Cost/reachability agreement: yes");
    }
    println!(
        "\nTiming is exploratory; use --release and repeated runs. Zero-cost edges are allowed."
    );
    Ok(())
}

fn main() -> ExitCode {
    match Options::parse(std::env::args().skip(1)) {
        Ok(None) => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        Ok(Some(options)) => match run(options) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Error: {error}");
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_defaults_and_explicit_options() {
        let options = Options::parse([]).unwrap().unwrap();
        assert_eq!(options.algorithm, Algorithm::Both);
        assert_eq!(options.start, "Condo for sale");
        let options = Options::parse(
            [
                "--data",
                "custom.csv",
                "--start",
                "A",
                "--end",
                "B",
                "--algorithm",
                "scan",
                "--repeat",
                "10",
                "--list-types",
            ]
            .map(str::to_owned),
        )
        .unwrap()
        .unwrap();
        assert_eq!(options.data, PathBuf::from("custom.csv"));
        assert_eq!(options.start, "A");
        assert_eq!(options.end, "B");
        assert_eq!(options.algorithm, Algorithm::Scan);
        assert_eq!(options.repeat, 10);
        assert!(options.list_types);
    }

    #[test]
    fn cli_rejects_invalid_arguments() {
        for args in [
            vec!["--algorithm", "bfs"],
            vec!["--repeat", "0"],
            vec!["--repeat", "text"],
            vec!["--data"],
            vec!["--start", "--end"],
            vec!["--unknown"],
        ] {
            assert!(Options::parse(args.into_iter().map(str::to_owned)).is_err());
        }
        assert!(Options::parse(["--help".into()]).unwrap().is_none());
    }
}
