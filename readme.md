# dpss: Reconciliation Engine & Subset Sum Solver

![github](https://user-images.githubusercontent.com/38364983/160049852-dbbf4d5a-1d48-4fb7-af0e-89efbd79c2e2.jpg)

[![Downloads](https://static.pepy.tech/personalized-badge/dpss?period=total&units=none&left_color=grey&right_color=brightgreen&left_text=PyPI%20Downloads)](https://pepy.tech/project/dpss)
[![PyPI - Downloads](https://img.shields.io/pypi/dd/dpss?label=PyPI%20Downloads%20%28Without%20Mirrors%29)](https://pypistats.org/packages/dpss)
[![Crates.io](https://img.shields.io/crates/d/subset_sum?label=crates.io%20Downloads)](https://crates.io/crates/subset_sum)
[![Crates.io (recent)](https://img.shields.io/crates/dr/subset_sum?label=crates.io%20Downloads%20%28recent%29)](https://crates.io/crates/subset_sum)
[![GitHub all releases](https://img.shields.io/github/downloads/europeanplaice/subset_sum/total?label=GitHub%20releases%20Downloads)](https://tooomm.github.io/github-release-stats/?username=europeanplaice&repository=subset_sum)
[![GitHub Repo stars](https://img.shields.io/github/stars/europeanplaice/subset_sum?style=social)](https://github.com/europeanplaice/subset_sum)


`dpss` (Dynamic Programming Subset Sum) is an ultra-fast algorithm library and **transaction reconciliation engine**. 
It solves the **many-to-many transaction matching problem** (also known as the *Subset Sum Matching Problem*) using an optimized dynamic programming approach.

Unlike standard matching tools that only handle 1-to-1 relationships, `dpss` can find combinations like **3 invoices that perfectly match 2 payments**, even with allowed variance (tolerance) for things like transaction fees or currency rounding.

## 🚀 Key Features

- **Reconciliation Engine**: Direct support for transaction objects (ID, Amount, Date, Description).
- **Tolerance Matching**: Match transactions even if amounts differ slightly (e.g., due to bank fees).
- **Extremely Fast**: Uses a sparse hash-set-based DP table and runs in parallel via Rayon.
- **Cross-Platform**: Available as a Rust Crate, a Python Package, a CLI tool, and a WebAssembly app.

## 💼 Use Cases

### Bank & Account Reconciliation
Bank statements and internal ledgers often diverge. A single payment on the bank side might map to multiple entries in the ledger, or vice versa. `dpss` finds exactly these many-to-many correspondences, surfacing what remains unmatched so you know exactly where the discrepancy lies.

### The "Subset Sum Matching Problem" (SSMP)
This specific challenge in automated banking was recently formalized as a combinatorial optimization task by J.P. Morgan AI Research (ECAI 2025). `dpss` provides an out-of-the-box open-source engine to solve exactly this.

## 🛠️ Usage

There are five ways to use this program:
* [Web UI](#web-ui)🌎 (The easiest way!)
* [Python](#python)🐍
* [CLI](#cli)🖥️
* [Rust](#rust)🦀
* [Agent Skills / MCP](#agent-skills-mcp)🤖

---

## <a id="web-ui"></a>Web UI (WASM)
You can run the reconciliation engine directly in your browser without uploading data to any server.

1. **[Reconciliation Engine Web UI](https://europeanplaice.github.io/subset_sum/reconcile)**: Upload two CSV files (Keys and Targets) and instantly get a matched groups report.
2. [Subset Sum Solver Web UI](https://europeanplaice.github.io/subset_sum/find_subset): Raw array-based subset sum matching.

---

## <a id="python"></a>Use in Python

```bash
pip install dpss
```

### Transaction Reconciliation
The fastest way to match Pandas DataFrames or lists of dictionaries.

```python
import dpss

# 1. Define your data
keys = [
    dpss.Transaction("k1", 10300), # e.g., $103.00
    dpss.Transaction("k2", 19800),
    dpss.Transaction("k3", 50000),
]

targets = [
    dpss.Transaction("t1", 10000), 
    dpss.Transaction("t2", 20000),
    dpss.Transaction("t3", 50000),
]

# 2. Run Reconciliation
# max_key_group=5, max_target_group=5, tolerance=500 (allow up to $5.00 diff)
result = dpss.reconcile(keys, targets, 5, 5, 500)

print(f"Matched amount: {result.summary.matched_amount}")
for group in result.matched:
    print(f"Match: {[k.id for k in group.keys]} == {[t.id for t in group.targets]} (Diff: {group.difference})")

# Output:
# Matched amount: 80100
# Match: ['k1'] == ['t1'] (Diff: 300)
# Match: ['k2'] == ['t2'] (Diff: -200)
# Match: ['k3'] == ['t3'] (Diff: 0)
```

### Raw Algorithm Access
```python
# Find subsets that sum to a target
print(dpss.find_subset([1, -2, 3, 4, 5], 2, 3))
# [[4, -2], [3, -2, 1]]
```

---

## <a id="cli"></a>CLI

Download the binary from the [Releases](https://github.com/europeanplaice/subset_sum/releases) page.

### 1. CSV Reconciliation
Given two CSV files with headers `id,amount,date,description`:

```bash
subset_sum reconcile keys.csv targets.csv --tolerance 5
```

### 2. Sequence Matcher
Find matching subsets from two text files containing raw numbers.
```bash
subset_sum sequence-matcher arr1.txt arr2.txt 10 10
```

---

## <a id="rust"></a>Use in Rust

Add it to your `Cargo.toml`:
```toml
[dependencies]
dpss = { version = "0.23.3", package = "subset_sum" }
```

```rust
use dpss::reconciliation::{reconcile, Transaction, ReconciliationConfig};

fn main() {
    let keys = vec![
        Transaction { id: "k1".into(), amount: 100, date: None, description: None },
        Transaction { id: "k2".into(), amount: 200, date: None, description: None },
    ];
    let targets = vec![
        Transaction { id: "t1".into(), amount: 300, date: None, description: None },
    ];
    
    let config = ReconciliationConfig {
        max_key_group_size: 5,
        max_target_group_size: 5,
        tolerance: 0,
        n_candidates: 10,
    };

    let result = reconcile(keys, targets, config).unwrap();
    println!("Matched groups: {}", result.matched.len());
}
```

---

## <a id="agent-skills-mcp"></a>Use with Agent Skills / MCP
`dpss` includes a Model Context Protocol (MCP) server and specialized Agent Skills, allowing AI assistants like Claude Code and Gemini CLI to autonomously reconcile your data safely and deterministically.

- See [`python_mcp/README.md`](python_mcp/README.md) for MCP server setup instructions.
- We provide an official agent skill instruction set in [`.agents/skills/dpss-reconcile/SKILL.md`](.agents/skills/dpss-reconcile/SKILL.md).