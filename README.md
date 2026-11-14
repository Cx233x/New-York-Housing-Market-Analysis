# New York Housing Market Analysis

**Rust · Graph modeling · Two Dijkstra implementations**

A command-line project that turns New York housing listing categories into an
undirected graph using overlapping price ranges. It reports connectivity and
edge-weight statistics, displays minimum-cost routes, and compares a binary-heap
Dijkstra implementation with a linear-scan implementation.

Originally developed for **DS210: Programming for Data Science at Boston University**.
The [original project writeup](doc/DS210%20Final%20project%20Writeup.pdf) is a historical
reference; the definitions and behavior below describe the current implementation.

## Getting started

Requires Rust **1.76 or later** and Cargo. Install them from [rustup.rs](https://rustup.rs/).

```bash
git clone https://github.com/Cx233x/New-York-Housing-Market-Analysis.git
cd New-York-Housing-Market-Analysis
cargo run --locked --release
```

The default query compares both algorithms from `Condo for sale` to `Co-op for sale`.
The program prints sorted category and edge tables, a timing table, and the full
route returned by each implementation.

```bash
# Discover valid category names.
cargo run --locked -- --list-types

# Compare both implementations using repeated queries.
cargo run --locked --release -- \
  --data src/NY-House-Dataset.csv \
  --start "Condo for sale" \
  --end "Co-op for sale" \
  --algorithm both \
  --repeat 1000

# Select one implementation.
cargo run --locked --release -- --algorithm scan

# Show all options.
cargo run --locked -- --help
```

| Option | Default | Meaning |
|---|---|---|
| `--data PATH` | Bundled `src/NY-House-Dataset.csv` | CSV path; an explicit relative path is relative to the working directory |
| `--start TYPE` | `Condo for sale` | Exact category name after whitespace trimming |
| `--end TYPE` | `Co-op for sale` | Exact destination category name |
| `--algorithm NAME` | `both` | `heap`, `scan`, or `both` |
| `--repeat N` | `1` | Positive number of timed queries per implementation |
| `--list-types` | Off | Print sorted category names and exit |
| `-h`, `--help` | — | Print usage and exit |

Unknown categories, invalid options, malformed prices, missing columns, and files
without usable prices produce an error and a nonzero exit status. Known but
disconnected categories produce an `unreachable` result. Custom CSV files must
contain `TYPE` and `PRICE` headers; other columns are ignored. Prices must be
nonnegative integers fitting in `u64`. Empty prices are skipped, whitespace is
trimmed, and empty category names are rejected. A category with no valid prices
is excluded; a category with valid prices but no edges is retained.

## Graph definition

For each category `T`, collect its minimum and maximum observed listing prices:

```text
range(T) = [min_price(T), max_price(T)]
```

- **Nodes:** Distinct `TYPE` values with at least one valid price.
- **Edges:** A pair of different categories whose closed price intervals overlap
  or touch. There are no self-loops. Both directions are stored internally.
- **Weight:** Width of the shared interval in USD:

```text
weight(A, B) = min(max_price(A), max_price(B))
             - max(min_price(A), min_price(B))
```

This formula is used only when the intervals overlap or touch. Touching intervals
and overlapping single-price ranges produce zero-weight edges.

A path's cost is the **sum of its overlap widths**. A minimum-cost route therefore
minimizes cumulative overlap width. It is not a geographic route, a transaction
price, a transition budget, or a measure of maximum housing similarity. In
particular, larger overlaps can indicate stronger price-range relationships while
the shortest-path objective favors smaller weights.

This update preserves the original model to make its behavior inspectable.
Changing to normalized similarity costs or percentile-based ranges would be a
separate modeling experiment with different interpretations and results.

## Verified results for the bundled snapshot

| Metric | Value |
|---|---:|
| CSV rows, excluding header | 4,801 |
| Categories | 13 |
| Unique undirected edges | 77 of 78 possible |
| Mean degree | 11.846154 |
| Population degree variance | 0.130178 |
| Degree standard deviation | 0.360801 |
| Degree mode | 12 |
| Zero-weight undirected edges | 11 |
| House for sale ↔ Townhouse for sale overlap | $64,685,000 |
| Coming Soon ↔ Foreclosure overlap | $1,046,000 |

Both algorithms return this route for the default query:

```text
Condo for sale -> Mobile house for sale -> Co-op for sale
Cost: 0
```

`Mobile house for sale` has one listing at $1,288,000. Its interval has zero width,
so overlapping categories can be connected through zero-cost edges. This result
illustrates a limitation of the weight definition, not a failure of Dijkstra.

Statistics labeled **overlap width** summarize graph weights, not individual
listing prices. Overall edge statistics count each undirected edge once.
Per-category minimum/mean/maximum overlap statistics summarize incident edges;
isolated nodes display `N/A`. Output rows and modes are sorted.

## Algorithm comparison

Both implementations solve the same nonnegative-weight shortest-path problem:

| Implementation | Frontier selection | Expected scaling for simple graphs* |
|---|---|---|
| `dijkstra_heap` | Binary heap with lazy distance updates | `O((V + E) log V)` |
| `dijkstra_linear_scan` | Scan the frontier for the smallest distance | `O(V² + E)` |

*These conventional bounds assume constant-size vertex identifiers and expected
constant-time hash access. This project stores and clones category strings, which
also contributes overhead. The heap implementation skips stale entries.

Both return a cost and the ordered route. Equal frontier costs are resolved by
category name. Equal-cost alternative routes are valid; correctness comparisons
use minimum cost and reachability rather than requiring identical routes.
Distance arithmetic is checked, and an examined sum exceeding `u64::MAX` is an
explicit error. A distance exactly equal to `u64::MAX` remains representable.

With `--algorithm both`, the program checks cost/reachability agreement and exits
with an error if they differ. Agreement between two implementations is not an
independent proof of correctness; tests also use known answers and a separate
Floyd–Warshall reference on small generated graphs.

### Timing interpretation

`--repeat N` performs one untimed warm-up followed by `N` queries per algorithm.
The displayed mean is elapsed nanoseconds divided by `N`. CSV loading, graph
construction, statistics, and terminal output are excluded. Search allocations,
route reconstruction, and loop overhead are included. The heap implementation
runs first when comparing both.

These are exploratory local timings, not published benchmark claims. Use release
builds, repeat the whole command, and record your CPU, Rust version, graph size,
query, and repetition count. The real graph has only 13 nodes; overhead can dominate
and the scan implementation may be faster. A scaling study should add seeded
synthetic graphs across sizes and sparse/dense connectivity, varied query pairs,
and varied execution order. Runtime comparisons on those larger graphs have not
yet been performed.

## Validation and continuous integration

```bash
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

GitHub Actions runs these checks on pushes and pull requests using stable Rust.
Tests cover both algorithms, route costs, cheaper indirect routes, unreachable and
unknown nodes, equal-cost alternatives, zero-weight cycles, overflow, CSV schema
and parsing, isolated categories, and statistical edge cases. They also compare
both implementations against Floyd–Warshall on fixed-seed small graphs and against
each other for all **169 ordered category pairs** in the bundled graph.

## Data limitations

- Minimum/maximum ranges are sensitive to outliers and do not describe the full
  listing-price distribution. The graph is almost complete, limiting how much its
  connectivity alone can explain.
- `TYPE` mixes property types with listing statuses such as `Pending`, `Contingent`,
  and `Coming Soon`.
- The CSV contains **214 exact duplicate rows beyond their first occurrence**.
  Duplicates do not affect the current range-based graph but would affect
  listing-level statistics.
- A house price of **$2,147,483,647** needs investigation; no outlier removal or
  deduplication is performed.
- Single-listing categories create degenerate ranges and may yield zero-cost
  routes. The model is descriptive and does not support causal or investment claims.
- The bundled snapshot is historical; its original download date was not recorded.

## License and dataset rights

Project code and project-authored documentation use the [MIT License](LICENSE).
The third-party CSV is **not** covered by that license.

Source: [New York Housing Market by Nidula Elgiriyewithana on Kaggle](https://www.kaggle.com/datasets/nelgiriyewithana/new-york-housing-market).
The official metadata labels its license “Other (specified in description)”; clear
redistribution terms were not found in the retrieved description. **Permission to
redistribute the bundled CSV has not been confirmed.** Resolve this before publicly
publishing a repository or release containing the data. See [DATASET.md](DATASET.md)
for provenance, the snapshot checksum, and the publication options.

## Project structure

```text
.github/workflows/ci.yml       Formatting, Clippy, and tests
src/main.rs                   CLI, sorted tables, and exploratory timings
src/graph.rs                  Heap and linear-scan Dijkstra, route reconstruction
src/parser.rs                 CSV validation and overlap graph construction
src/calculation.rs            Degree and undirected overlap-weight statistics
src/NY-House-Dataset.csv      Third-party dataset; see DATASET.md
doc/                          Original course project writeup
Cargo.toml / Cargo.lock        Package metadata and locked dependencies
LICENSE                       MIT license for project code
DATASET.md                    Data provenance and unresolved redistribution terms
```
