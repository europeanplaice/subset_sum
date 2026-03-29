use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufRead};

mod dp_module;
#[cfg(feature = "python")]
mod py_module;
mod reconciliation;

use dp_module::dp;
use reconciliation::{reconcile, ReconciliationConfig, Transaction};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solves the subset sum problem
    SubsetSum {
        /// The path of the text file containing the set of integers
        file: String,
        /// The target sum
        target: i64,
        /// The maximum length of the combination
        #[arg(default_value_t = usize::MAX)]
        max_length: usize,
    },
    /// Solves the sequence matcher problem
    SequenceMatcher {
        /// The keys text file path
        keys_file: String,
        /// The targets text file path
        targets_file: String,
        /// The max key length
        #[arg(default_value_t = usize::MAX)]
        max_key_length: usize,
        /// The max target length
        #[arg(default_value_t = usize::MAX)]
        max_target_length: usize,
        /// The maximum number of answers
        #[arg(default_value_t = 10)]
        n_candidates: usize,
        /// Use all keys
        #[arg(short = 'k', long, default_value_t = false)]
        use_all_keys: bool,
        /// Use all targets
        #[arg(short = 't', long, default_value_t = false)]
        use_all_targets: bool,
    },
    /// Reconciles two CSV files containing transactions
    Reconcile {
        /// The keys CSV file path
        keys_file: String,
        /// The targets CSV file path
        targets_file: String,
        /// Max key group size
        #[arg(short = 'k', long, default_value_t = 5)]
        max_key_group_size: usize,
        /// Max target group size
        #[arg(short = 't', long, default_value_t = 5)]
        max_target_group_size: usize,
        /// Tolerance for matching amounts (in the lowest denomination, e.g., cents)
        #[arg(long, default_value_t = 0)]
        tolerance: i64,
        /// The maximum number of answers internally generated
        #[arg(short = 'n', long, default_value_t = 10)]
        n_candidates: usize,
    },
}

#[derive(Debug, Deserialize)]
struct CsvRow {
    id: String,
    amount: i64,
    date: Option<String>,
    description: Option<String>,
}

fn read_lines(filename: &str) -> io::Result<Vec<i64>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut vec = Vec::new();
    for l in lines.flatten() {
        if !l.is_empty() {
            if let Ok(num) = l.trim().parse::<i64>() {
                vec.push(num);
            }
        }
    }
    Ok(vec)
}

fn read_transactions_csv(filename: &str) -> Result<Vec<Transaction>, csv::Error> {
    let mut reader = csv::Reader::from_path(filename)?;
    let mut transactions = Vec::new();
    for result in reader.deserialize() {
        let record: CsvRow = result?;
        transactions.push(Transaction {
            id: record.id,
            amount: record.amount,
            date: record.date,
            description: record.description,
        });
    }
    Ok(transactions)
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::SubsetSum {
            file,
            target,
            max_length,
        } => {
            if let Ok(arr) = read_lines(file) {
                let actual_max = if *max_length == usize::MAX {
                    arr.len()
                } else {
                    *max_length
                };
                let result = dp::find_subset(arr, *target, actual_max);
                println!("{:?}", result);
            } else {
                println!("Error reading file: {}", file);
            }
        }
        Commands::SequenceMatcher {
            keys_file,
            targets_file,
            max_key_length,
            max_target_length,
            n_candidates,
            use_all_keys,
            use_all_targets,
        } => {
            if let (Ok(mut keys), Ok(mut targets)) =
                (read_lines(keys_file), read_lines(targets_file))
            {
                let k_len = if *max_key_length == usize::MAX {
                    keys.len()
                } else {
                    *max_key_length
                };
                let t_len = if *max_target_length == usize::MAX {
                    targets.len()
                } else {
                    *max_target_length
                };
                match dp::sequence_matcher(
                    &mut keys,
                    &mut targets,
                    k_len,
                    t_len,
                    *n_candidates,
                    *use_all_keys,
                    *use_all_targets,
                ) {
                    Ok(result) => println!("{}", dp::sequence_matcher_formatter(result)),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Error reading one or both input files.");
            }
        }
        Commands::Reconcile {
            keys_file,
            targets_file,
            max_key_group_size,
            max_target_group_size,
            tolerance,
            n_candidates,
        } => {
            match (
                read_transactions_csv(keys_file),
                read_transactions_csv(targets_file),
            ) {
                (Ok(keys), Ok(targets)) => {
                    let config = ReconciliationConfig {
                        max_key_group_size: *max_key_group_size,
                        max_target_group_size: *max_target_group_size,
                        tolerance: *tolerance,
                        n_candidates: *n_candidates,
                    };
                    match reconcile(keys, targets, config) {
                        Ok(result) => {
                            println!("Reconciliation Summary:");
                            println!("-----------------------");
                            println!("Total Keys: {}", result.summary.total_keys);
                            println!("Total Targets: {}", result.summary.total_targets);
                            println!(
                                "Matched Keys: {}/{} (Amount: {})",
                                result.summary.matched_key_count,
                                result.summary.total_keys,
                                result.summary.matched_amount
                            );
                            println!(
                                "Matched Targets: {}/{}",
                                result.summary.matched_target_count, result.summary.total_targets
                            );
                            println!(
                                "Unmatched Keys Amount: {}",
                                result.summary.unmatched_key_amount
                            );
                            println!(
                                "Unmatched Targets Amount: {}",
                                result.summary.unmatched_target_amount
                            );
                            println!("\nMatched Groups:");
                            for (i, group) in result.matched.iter().enumerate() {
                                println!("  Group {}:", i + 1);
                                println!(
                                    "    Keys: {:?}",
                                    group.keys.iter().map(|k| k.id.clone()).collect::<Vec<_>>()
                                );
                                println!(
                                    "    Targets: {:?}",
                                    group
                                        .targets
                                        .iter()
                                        .map(|t| t.id.clone())
                                        .collect::<Vec<_>>()
                                );
                                println!(
                                    "    Key Sum: {}, Target Sum: {}, Diff: {}",
                                    group.key_sum, group.target_sum, group.difference
                                );
                            }
                            println!("\nUnmatched Keys: {}", result.unmatched_keys.len());
                            for k in result.unmatched_keys {
                                println!("  - {}: {}", k.id, k.amount);
                            }
                            println!("\nUnmatched Targets: {}", result.unmatched_targets.len());
                            for t in result.unmatched_targets {
                                println!("  - {}: {}", t.id, t.amount);
                            }
                        }
                        Err(e) => println!("Error during reconciliation: {}", e),
                    }
                }
                (Err(e), _) => println!("Error reading keys CSV: {}", e),
                (_, Err(e)) => println!("Error reading targets CSV: {}", e),
            }
        }
    }
}
