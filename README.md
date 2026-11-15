# New York Housing Market Analysis

**Rust | Graph Modeling | Dijkstra's Algorithm | Data Analysis**

A Rust-based command-line project exploring price-range relationships across New York housing listing categories through graph construction, descriptive statistics, and shortest-path algorithms.

Originally developed for **DS210: Programming for Data Science at Boston University**, the project has been reorganized into a reproducible, tested Rust application.

## Project Overview

This project converts housing listing categories into an undirected weighted graph based on overlapping price ranges.

It focuses on:

- Constructing a graph from housing listing data.
- Analyzing connectivity and edge-weight distributions.
- Comparing binary-heap and linear-scan implementations of Dijkstra's algorithm.
- Exploring how graph definitions affect the interpretation of housing market relationships.

The project models price-range overlap rather than geographic proximity, transaction costs, or causal relationships.

## Getting Started

### Prerequisites

- Rust 1.76 or later
- Cargo

Install Rust through [rustup](https://rustup.rs/).

### Installation

```bash
git clone https://github.com/Cx233x/nyc-housing-market-analysis.git
cd nyc-housing-market-analysis

cargo build --locked
```

### Run with Sample Data

A small synthetic dataset is provided for testing and demonstration. No external download is needed.

```bash
cargo run --locked -- \
  --data tests/fixtures/categories.csv \
  --start A \
  --end B \
  --algorithm both
```

The fixture contains three categories, A, B, and C.

- A and B have an overlapping price interval of width 2.
- C is an isolated node.
- Both Dijkstra implementations should return a minimum cost of 2 for A to B.

### Run with the Original Housing Dataset

The original third-party Kaggle dataset is **not included** in this repository because its redistribution permissions have not been confirmed.

1. Download [New York Housing Market](https://www.kaggle.com/datasets/nelgiriyewithana/new-york-housing-market) from Kaggle under the applicable terms.
2. Create a `data/` directory in the repository root.
3. Place the extracted CSV at `data/NY-House-Dataset.csv`.

Run the analysis:

```bash
cargo run --locked --release -- \
  --data data/NY-House-Dataset.csv \
  --start "Condo for sale" \
  --end "Co-op for sale" \
  --algorithm both \
  --repeat 1000
```

List available categories:

```bash
cargo run --locked -- --list-types
```

The original category names above may need to be adjusted if the upstream dataset has changed.

See [DATASET.md](DATASET.md) for provenance, schema requirements, and historical snapshot information.

## Command-Line Options

| Option | Default | Description |
|---|---|---|
| `--data PATH` | `data/NY-House-Dataset.csv` | Input CSV path |
| `--start TYPE` | `Condo for sale` | Starting category |
| `--end TYPE` | `Co-op for sale` | Destination category |
| `--algorithm NAME` | `both` | `heap`, `scan`, or `both` |
| `--repeat N` | `1` | Number of timed queries |
| `--list-types` | Off | Display available categories |
| `--help` | — | Show CLI instructions |

The CSV must include `TYPE` and `PRICE` columns. Prices must be nonnegative integers; empty prices are skipped.

## Graph Construction

For each housing category, the program calculates the minimum and maximum observed listing prices.

Each category becomes a graph node. Two different categories are connected when their closed price intervals overlap or touch.

The weight of an edge is the width of their shared price interval:

```text
weight(A, B) =
    min(max_price(A), max_price(B))
  - max(min_price(A), min_price(B))
```

An edge exists only if the intervals intersect.

The resulting graph is undirected and contains nonnegative weights. Touching intervals may create zero-weight edges.

A shortest path minimizes the sum of overlap widths. It should not be interpreted as the most similar sequence of property types: larger overlaps may indicate stronger price-range relationships, but Dijkstra minimizes the assigned weights.

## Historical Dataset Results

The following results were recorded from the original local dataset snapshot. The CSV is no longer distributed with this repository, and results may differ with other dataset versions.

| Metric | Result |
|---|---:|
| Listings (CSV rows) | 4,801 |
| Housing/listing categories | 13 |
| Unique undirected edges | 77 |
| Mean degree | 11.846154 |
| Population degree variance | 0.130178 |
| Degree standard deviation | 0.360801 |
| Degree mode | 12 |
| Zero-weight undirected edges | 11 |

Selected edge-weight observations:

- House for sale and Townhouse for sale: $64,685,000 overlap width.
- Coming Soon and Foreclosure: $1,046,000 overlap width.

For the historical snapshot, both algorithms found the following minimum-cost route:

```text
Condo for sale
    -> Mobile house for sale
    -> Co-op for sale

Cumulative overlap width: 0
```

The zero-cost route occurs because a single-listing category creates a price interval of zero width. This illustrates a modeling limitation rather than a shortest-path algorithm error.

## Algorithm Comparison

Two implementations of Dijkstra's algorithm are provided:

| Algorithm | Frontier selection | Conventional complexity |
|---|---|---|
| Binary-heap Dijkstra | Priority queue | O((V + E) log V) |
| Linear-scan Dijkstra | Linear search | O(V² + E) |

Both return minimum path costs and reconstructed routes.

The application compares their costs and reachability when `--algorithm both` is selected. Equal-cost paths do not necessarily have identical intermediate nodes.

### Timing Methodology

The `--repeat N` option performs one untimed warm-up followed by N timed queries per algorithm.

CSV loading, graph construction, and output formatting are excluded from the measured intervals.

These measurements are exploratory. The historical graph contains only 13 nodes, so timing overhead may outweigh asymptotic differences. No general performance superiority is claimed without larger, controlled benchmarks.

## Testing and Validation

The project includes unit tests, CLI integration tests, and GitHub Actions checks.

Run locally:

```bash
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Tests cover:

- CSV parsing and schema validation
- Graph construction and edge cases
- Known shortest-path results
- Unreachable and unknown categories
- Zero-weight edges and overflow handling
- Agreement between Dijkstra implementations
- Comparison with Floyd-Warshall on small generated graphs
- CLI argument parsing and execution

Automated tests use synthetic fixtures and do not require the original Kaggle dataset.

## Data and Modeling Limitations

- Per-category minimum and maximum prices are sensitive to outliers.
- Listing categories mix property types and market statuses.
- Highly overlapping intervals produce a nearly complete graph.
- Zero-width intervals may create zero-cost paths.
- Duplicate listings do not affect minimum/maximum ranges, but would affect listing-level summaries.
- Shortest paths in the constructed graph do not represent geographic routes, market transitions, or investment recommendations.
- Historical findings should not be interpreted as current NYC housing market conditions.

## Project Structure

```text
nyc-housing-market-analysis/
├── .github/
│   └── workflows/
│       └── ci.yml
├── docs/
│   └── project-report.pdf
├── src/
│   ├── calculation.rs
│   ├── graph.rs
│   ├── main.rs
│   └── parser.rs
├── tests/
│   ├── fixtures/
│   │   └── categories.csv
│   └── cli.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── DATASET.md
├── LICENSE
└── README.md
```

The optional `data/` directory is created locally by users and holds the untracked housing dataset.

## Documentation

The [original course project writeup](docs/DS210%20Final%20project%20Writeup.pdf) is retained as a historical reference. The current source code and README define the maintained implementation.

## License and Data Attribution

Project code and project-authored documentation are released under the [MIT License](LICENSE).

The original housing dataset was published on Kaggle by Nidula Elgiriyewithana. The dataset is not covered by this repository's MIT license and is not redistributed here.

See [DATASET.md](DATASET.md) for details.