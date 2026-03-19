# 🏙️ New York Housing Market Analysis

A graph-based analysis program written in **Rust** that explores the relationship between property types and their prices in the New York housing market.

---

## 📊 Dataset

**Source:** [New York Housing Market — Kaggle](https://www.kaggle.com/datasets/nelgiriyewithana/new-york-housing-market/data)

The dataset includes information on various types of properties such as multi-family homes, condos, co-ops, and more. In this project, **property types are modeled as nodes** and **prices as edges** in a graph structure.

---

## 📖 Introduction

This project analyzes the New York housing market using graph-based techniques. By treating property types as nodes and prices as edge weights, the program computes statistical measures — including degree distribution, mean, variance, standard deviation, and edge weight analysis — to uncover pricing dynamics and connectivity patterns across different property categories.

---

## 🚀 How to Run

> Make sure **Rust** and **Cargo** are installed on your system. You can install them at [rustup.rs](https://rustup.rs).

**Build the project:**
```bash
cargo build
```

**Run the program:**
```bash
cargo run
```

**Run tests:**
```bash
cargo test
```

---

## ✨ Key Features

### `main.rs`
- Entry point of the program
- Contains the `main` function, 3 testing functions, and module declarations

### `parser.rs`
- Parses the CSV dataset
- Constructs the graph structure for further analysis

### `calculation.rs`
- Performs computations on:
  - Degree distribution
  - Mean, variance, standard deviation, and mode
  - Edge weight analysis
  - Shortest path
  - Price distribution
  - Pricing differences

### `graph.rs`
- Implements **Dijkstra's Algorithm** for shortest path analysis between property types

---

## 🖥️ Program Outputs

### Degree Distribution
Measures how many connections each property type has.

| Metric | Value |
|---|---|
| Mean Degree | ~11.85 |
| Variance | 0.13 |
| Standard Deviation | 0.36 |
| Mode | 12 |

The low variance and standard deviation indicate a **consistent level of connectivity** across property types, with most having approximately 12 connections.

---

### Edge Weight Analysis
Outputs the weights (prices) of connections between different property types, revealing significant price variation.

**Examples:**
- `Townhouse for sale` ↔ `House for sale` → **$64,685,000**
- `Coming Soon` ↔ `Foreclosure` → **$1,046,000**

Average, median, and maximum prices vary widely depending on the property types involved.

---

### Price Distribution
Outputs the distribution of prices across all property types in the dataset.

---

### Price Differences
Outputs the full price range for each individual property type.

---

## 🧪 Tests

Run the test suite with:
```bash
cargo test
```

The `main.rs` file contains 3 built-in test functions to validate core functionality.

---

## 📝 Conclusion

The analysis reveals a **diverse real estate market** where property type significantly influences price:

- High-priced categories like **"Condo for sale"** and **"House for sale"** contrast sharply with low-priced options like **"Mobile house for sale"**
- This wide range suggests a market that caters to buyers across all budget levels
- The connectivity patterns among property types reflect a **closely knit market**, where different property categories are highly interconnected

---

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) — Systems programming language
- [Cargo](https://doc.rust-lang.org/cargo/) — Rust package manager and build system

---

## 📂 Project Structure

```
├── src/
│   ├── main.rs          # Entry point, tests, and module usage
│   ├── parser.rs        # CSV parser and graph builder
│   ├── calculation.rs   # Statistical calculations and analysis
│   └── graph.rs         # Dijkstra's shortest path algorithm
├── Cargo.toml
└── README.md
```
