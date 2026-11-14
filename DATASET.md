# Dataset provenance and licensing

The repository's MIT license applies to the project code and project-authored
documentation. It does **not** relicense `src/NY-House-Dataset.csv` or grant rights
to third-party data.

## Source

- Title: **New York Housing Market**
- Publisher: **Nidula Elgiriyewithana** (`nelgiriyewithana`)
- Dataset: https://www.kaggle.com/datasets/nelgiriyewithana/new-york-housing-market
- Official metadata: https://www.kaggle.com/api/v1/datasets/view/nelgiriyewithana/new-york-housing-market
- Upstream version reported by the metadata: **1**, last updated **2024-01-06**.
- Metadata checked: **2026-11-14**.

The historical download date and exact upstream version of the local CSV were
not recorded. The metadata above describes the upstream dataset; it does not
establish that the bundled file is byte-identical to that version.

## Redistribution status: not confirmed

The official metadata currently labels the license **“Other (specified in
description)”**. The retrieved description lists fields and potential uses but
provides no explicit redistribution grant or concrete license terms. Public
availability on Kaggle does not establish permission to redistribute this CSV.

The existing CSV has been retained locally without alteration for reproducibility;
this notice is not a substitute for permission. Before publishing a repository,
release, or package containing the CSV:

1. Obtain and retain explicit applicable terms or permission from the data owner.
2. Follow any attribution, notice, commercial-use, or redistribution conditions
   specified in those terms. These conditions have not yet been established.
3. If permission cannot be established, distribute the code without the CSV and
   direct users to obtain data from the source under its applicable terms.
   Removing a tracked file from the working tree alone does not remove it from
   Git history or previously created releases.

No permission request has been sent to the publisher by this project update.

## Local snapshot

- Path: `src/NY-House-Dataset.csv`
- Rows excluding header: **4,801**
- Columns: **17**; the program reads `TYPE` and `PRICE`.
- Exact duplicate rows beyond their first occurrence: **214**.
- SHA-256: `87a3bd9b534a707ff4b3badf5a3c8d7e144d725efb7abbf0d7d1af3b896a57c1`

The application does not deduplicate or remove outliers. Duplicate rows do not
change per-category minimum/maximum ranges. They would affect listing counts and
listing-level distributions if those features were added. A house price of
`2,147,483,647` warrants investigation. `TYPE` mixes housing types and listing
statuses. The snapshot should not be described as current market data or completed
transaction data.
