mod dp_module;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(args[1].clone()).unwrap();
    let lines = io::BufReader::new(file).lines();
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