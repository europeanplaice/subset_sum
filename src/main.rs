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
    if Path::new(&args[2]).exists(){
        let mut key: Vec<i32> = Vec::new();
        for line in lines{
            // Todo: skip an empty line
            key.push(line.unwrap().trim().parse::<i32>().unwrap());
        }
        let file = File::open(args[2].clone()).unwrap();
        let line2 = io::BufReader::new(file).lines();
        let mut targets: Vec<i32> = Vec::new();
        for line in line2{
            // Todo: skip an empty line
            targets.push(line.unwrap().trim().parse::<i32>().unwrap());
        }
        if args.len() >= 4 && args[3] == "m2m"{
            let n_candidates = if args.len() == 4 {
                10
            } else {
                args[4].parse::<usize>().unwrap()
            };
            let max_key_length = if args.len() == 4 || args.len() == 5 {
                2
            } else {
                args[5].parse::<usize>().unwrap()
            };
            let result = dp_module::dp::sequence_matcher_m2m(&mut key, &mut targets, n_candidates, max_key_length);
            for elem in result{
                println!("{:?}", elem);
            }
        } else {
            let result = dp_module::dp::sequence_matcher(&mut key, &mut targets);
            for elem in result{
                println!("{:?}", elem);
            }

        }
    } else {
        let mut a: Vec<i32> = Vec::new();
        for line in lines{
            a.push(line.unwrap().trim().parse::<i32>().unwrap());
        }
        if a.iter().min().unwrap() >= &0 {
            let b: Vec<u32> = a.iter().map(|x| *x as u32).collect();
            println!("{:?}", dp_module::dp::find_subset_fast_only_positive(&b, args[2].parse::<usize>().unwrap()));
        } else {
            let result = dp_module::dp::find_subset(&a, args[2].parse::<i32>().unwrap());
            println!("{:?}", result);
        }
    }
}