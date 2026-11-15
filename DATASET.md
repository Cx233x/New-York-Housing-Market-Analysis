# Dataset Provenance and Licensing

This document describes the origin, usage, historical snapshot, and redistribution status of the dataset used in this project.

## 1. Data Source

- **Dataset:** New York Housing Market
- **Publisher:** Nidula Elgiriyewithana (`nelgiriyewithana`)
- **Platform:** Kaggle
- **Source:** https://www.kaggle.com/datasets/nelgiriyewithana/new-york-housing-market
- **Metadata:** https://www.kaggle.com/api/v1/datasets/view/nelgiriyewithana/new-york-housing-market

As checked on November 14, 2026, the upstream metadata reported:

- Dataset version: 1
- Last updated: January 6, 2024
- License label: Other (specified in description)

The original local download date and exact dataset version were not recorded. The upstream metadata does not establish that the historical local CSV is byte-identical to the listed upstream version.

## 2. Redistribution Status

**The original Kaggle dataset is not included in this repository.**

The retrieved Kaggle metadata and description did not establish an explicit redistribution grant.

Public availability on Kaggle does not automatically permit redistribution.

Accordingly:

- The repository distributes source code, tests, and project documentation, but not the original CSV.
- Users must obtain the dataset directly from Kaggle under its applicable terms.
- The repository's MIT license does not apply to third-party datasets.
- No permission request has been sent to the original publisher as part of this project update.

Removing the file from the current Git tree does not necessarily remove copies from earlier commits, forks, or releases.

## 3. Local Dataset Setup

Download the dataset from the Kaggle source above.

Create a directory named `data` in the repository root and place the CSV at:

```text
data/NY-House-Dataset.csv
```

Run the application:

```bash
cargo run --locked --release -- \
  --data data/NY-House-Dataset.csv
```

The `data/NY-House-Dataset.csv` path is excluded from Git tracking by `.gitignore`.

For a demonstration that does not require the original dataset, use:

```bash
cargo run --locked -- \
  --data tests/fixtures/categories.csv \
  --start A --end B
```

## 4. Required Schema

The application reads two columns:

| Column | Description |
|---|---|
| `TYPE` | Housing or listing category |
| `PRICE` | Nonnegative integer listing price |

Additional columns are ignored.

The parser trims whitespace, skips empty prices, and rejects malformed prices, missing required columns, and empty category names.

Categories without valid prices are excluded.

## 5. Historical Local Snapshot

The following information describes the original dataset snapshot used in the project. The snapshot is no longer committed to the repository.

| Property | Historical value |
|---|---|
| CSV rows (excluding header) | 4,801 |
| Columns | 17 |
| Exact duplicate rows beyond first occurrences | 214 |
| Graph categories | 13 |

**Historical SHA-256 checksum:**

```text
87a3bd9b534a707ff4b3badf5a3c8d7e144d725efb7abbf0d7d1af3b896a57c1
```

The checksum is provided to help identify the historical snapshot. It does not imply that the latest upstream dataset is identical.

## 6. Data Quality and Limitations

Several characteristics affect how the dataset should be interpreted:

- The `TYPE` field mixes property types and listing statuses.
- Exact duplicate rows exist in the historical snapshot.
- A recorded house price of $2,147,483,647 warrants further investigation.
- The current graph construction uses category-level minimum and maximum prices, making it sensitive to outliers.
- Duplicate records do not change the current category price ranges, but would affect listing-level statistics.
- The project does not perform outlier removal or deduplication.
- The historical snapshot is not necessarily representative of current market conditions.
- Listing prices should not be interpreted as completed transaction prices.

The project is intended for algorithmic and descriptive analysis, not property valuation, investment recommendations, or causal inference.

## 7. Testing Without the Dataset

Synthetic test data is available at:

```text
tests/fixtures/categories.csv
```

All automated tests are designed to run without the original Kaggle CSV.

```bash
cargo test --locked
```

The sample fixture exists solely for reproducible testing and demonstration.

## 8. License

The repository's MIT license applies to project code and project-authored documentation.

The original Kaggle dataset retains its own applicable terms and is not relicensed by this repository.

For project usage instructions, see [README.md](README.md).