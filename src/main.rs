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
        if args[3] == "m2m"{
            let n_candidates = if args.len() == 4 {
                10
            } else {
                args[4].parse::<usize>().unwrap()
            };
            let result = dp_module::dp::sequence_matcher_m2m(&mut key, &mut targets, n_candidates);
            for elem in result{
                println!("{:?}", elem);
            }
        } 
        if args[3] == "o2m" {
            let max_target_length = if args.len() == 4 {
                targets.len()
            } else {
                args[4].parse::<usize>().unwrap()
            };
            let result = dp_module::dp::sequence_matcher(&mut key, &mut targets, max_target_length);
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
            let max_length = if args.len() == 3 {
                b.len()
            } else {
                args[3].parse::<usize>().unwrap()
            };
            println!("{:?}", dp_module::dp::find_subset_fast_only_positive(&b, args[2].parse::<usize>().unwrap(), max_length));
        } else {
            let max_length = if args.len() == 3 {
                a.len()
            } else {
                args[3].parse::<usize>().unwrap()
            };
            let result = dp_module::dp::find_subset(&a, args[2].parse::<i32>().unwrap(), max_length);
            println!("{:?}", result);
        }
    }
}