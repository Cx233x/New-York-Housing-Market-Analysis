# New York Housing Market Analysis

**Rust | Graph Algorithms | Data Analysis | Statistical Computing**

A graph-based analysis of the New York housing market using **Rust** to investigate property price ranges, relationships between housing categories, and market connectivity.

The project transforms housing data into a weighted graph and applies statistical analysis and shortest-path algorithms to explore pricing patterns across different property types.

## 1. Project Overview

The goal of this project is to explore how property categories in New York's housing market relate to one another based on their price ranges.

**Key questions:**

- How do price ranges differ across property types?
- Which property categories have overlapping price ranges?
- How connected are different housing categories?
- What patterns can graph algorithms and statistical measures reveal?

### Dataset

**Source:** [New York Housing Market — Kaggle](https://www.kaggle.com/datasets/nelgiriyewithana/new-york-housing-market/data)

The dataset contains housing listings across multiple property categories, including condos, co-ops, townhouses, and multi-family properties.

The program uses the `TYPE` and `PRICE` columns to construct a graph representing relationships between housing categories.

## 2. Methodology

### Graph Construction

The housing dataset is transformed into a **weighted graph**:

- **Nodes:** Property types, such as condos or townhouses.
- **Edges:** Connections between property types whose price ranges overlap.
- **Edge Weights:** The width of the overlapping price interval.

For example, if two property categories have overlapping price ranges, an edge connects them. The edge weight measures the size of that overlap.

This allows the program to analyze relationships between property categories through graph-based techniques.

### Key Analysis Features

| Feature | Description |
|---|---|
| Graph Construction | Builds a weighted graph from property price ranges |
| Degree Distribution | Measures how many other property types each category connects to |
| Statistical Analysis | Calculates mean, variance, standard deviation, and mode of node degrees |
| Edge Weight Analysis | Summarizes the overlap weights between connected property categories |
| Shortest-Path Analysis | Finds minimum-weight paths between property categories |
| Price-Range Analysis | Summarizes the distribution and range of graph edge weights |

The project also includes an implementation of **Dijkstra's Algorithm** using a priority queue in `graph.rs`.

## 3. Key Findings

### Graph Connectivity

The original analysis reported the following degree distribution statistics:

| Metric | Value |
|---|---|
| Mean Degree | ~11.85 |
| Variance | 0.13 |
| Standard Deviation | 0.36 |
| Mode | 12 |

These results suggest that property categories in the constructed graph have similar numbers of connections, with most connected to approximately 12 other categories.

### Price-Range Overlap

The analysis also identified substantial differences in overlap weights between property categories.

Examples reported in the original analysis include:

| Property Types | Edge Weight |
|---|---:|
| Townhouse for sale ↔ House for sale | $64,685,000 |
| Coming Soon ↔ Foreclosure | $1,046,000 |

*Note: Edge weights represent overlapping price intervals, not individual property transaction prices.*

### Interpretation

The graph reveals a high degree of connectivity among housing categories and substantial variation in the widths of their overlapping price ranges.

These findings illustrate how graph-based modeling can help describe housing market structure beyond traditional summary statistics.

*The reported results are from the original project documentation and should be revalidated against the current dataset and implementation.*

## 4. Getting Started

### Prerequisites

Install [Rust and Cargo](https://rustup.rs/) before running the project.

### Step 1: Clone the Repository

```bash
git clone https://github.com/Cx233x/New-York-Housing-Market-Analysis.git
cd New-York-Housing-Market-Analysis
```

Alternatively, download the repository using **Code → Download ZIP** on GitHub.

### Step 2: Build and Run

Compile the project:

```bash
cargo build
```

Run the analysis:

```bash
cargo run
```

The program prints graph statistics, edge weight analysis, shortest-path results, and price-range summaries in the terminal.

**Important:** Before running the current version, update the hard-coded CSV path in `src/main.rs` to reference the dataset included in the repository.

### Step 3: Run Tests

```bash
cargo test
```

The project contains three unit tests covering degree distribution, edge weight analysis, and shortest-path calculations.

## 5. Project Structure

```text
New-York-Housing-Market-Analysis/
│
├── src/
│   ├── main.rs                 # Program entry point and tests
│   ├── parser.rs               # CSV parsing and graph construction
│   ├── calculation.rs          # Statistical and graph analysis
│   ├── graph.rs                # Dijkstra's algorithm
│   └── NY-House-Dataset.csv    # Housing dataset
│
├── doc/
│   └── DS210 Final project Writeup.pdf
│
├── Cargo.toml                  # Dependencies and project configuration
├── Cargo.lock                  # Dependency lockfile
└── README.md
```

## 6. Technologies Used

- **Rust:** Core programming language for data processing and graph algorithms
- **Cargo:** Dependency management and project compilation
- **csv / serde:** CSV parsing and data deserialization
- **itertools:** Iterator-based data processing
- **Standard Library Collections:** HashMap, HashSet, and BinaryHeap for graph representation and algorithm implementation

## 7. Limitations and Future Improvements

The graph models relationships between property categories using their minimum and maximum observed prices. These ranges do not capture the full distribution of individual listing prices.

Additionally, the shortest-path weights represent price-range overlap, so they should not be interpreted as geographical distance or travel cost.

Potential improvements include:

- Validating results against alternative graph representations
- Adding visualizations of property relationships and price distributions
- Expanding automated testing
- Improving handling of missing values and outliers
- Evaluating more informative measures of price similarity

## 8. Project Background

Developed as a final project for **DS210: Programming for Data Science at Boston University**.

For additional implementation details and analysis, see the [project writeup](doc/DS210%20Final%20project%20Writeup.pdf).
