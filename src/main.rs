mod dp_module;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(args[1].clone()).unwrap();
    let lines = io::BufReader::new(file).lines();
    if Path::new(&args[2]).exists() {
        let mut key: Vec<i64> = Vec::new();
        for line in lines {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    } else {
                        key.push(line.trim().parse::<i64>().unwrap())
                    }
                }
                Err(_) => println!("Error reading file"),
            }
        }
        let file = File::open(args[2].clone()).unwrap();
        let line2 = io::BufReader::new(file).lines();
        let mut targets: Vec<i64> = Vec::new();
        for line in line2 {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    } else {
                        targets.push(line.trim().parse::<i64>().unwrap())
                    }
                }
                Err(_) => println!("Error reading file"),
            }
        }
        let max_key_length = if args.len() == 3 {
            key.len()
        } else {
            args[3].parse::<usize>().unwrap()
        };
        let max_target_length = if args.len() == 4 {
            targets.len()
        } else {
            args[4].parse::<usize>().unwrap()
        };
        let n_candidates = if args.len() == 5 {
            10
        } else {
            args[5].parse::<usize>().unwrap()
        };
        let with_all_keys = if args.len() == 6 {
            false
        } else {
            args[6].parse::<bool>().unwrap()
        };
        let with_all_targets = if args.len() == 7 {
            false
        } else {
            args[7].parse::<bool>().unwrap()
        };
        let result: Vec<dp_module::dp::AnswerElement> = dp_module::dp::sequence_matcher(
            &mut key,
            &mut targets,
            max_key_length,
            max_target_length,
            n_candidates,
            with_all_keys,
            with_all_targets,
        )
        .unwrap();
        println!("{}", dp_module::dp::sequence_matcher_formatter(result));
    } else {
        let mut a: Vec<i64> = Vec::new();
        for line in lines {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    } else {
                        a.push(line.trim().parse::<i64>().unwrap())
                    }
                }
                Err(_) => println!("Error reading file"),
            }
        }
        let max_length = if args.len() == 3 {
            a.len()
        } else {
            args[3].parse::<usize>().unwrap()
        };
        let result = dp_module::dp::find_subset(a, args[2].parse::<i64>().unwrap(), max_length);
        println!("{:?}", result);
    }
}
