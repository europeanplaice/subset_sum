# Subset Sum

This is a Rust implementation that calculates subset sum problem. It returns sets of integers that sum up to a target value.

## Installation
Binary files are provided on the [Releases](https://github.com/europeanplaice/subset_sum/releases) page. When you download one of these, please add it to your PATH manually.

## Usage

First, you need to prepare a text file containing a set of integers like this
```
1
2
-3
4
5
```
and save it at any place.

Second, call `subset_sum` with the path of the text file and the target sum.  

### Example 
Call `subset_sum.exe num_set.txt 3`  
The executable's name `subset_sum.exe` would be different from your choice. Change this example along with your environment.

In this example, the output is   
`[[1, 2], [2, -3, 4], [1, -3, 5]]`

## Use in Rust

`Cargo.toml`
```
[dependencies]
subset_sum = "(version)"
```
Example
```
subset_sum = "0.5.0"
```

`main.rs`
```rust
use subset_sum::dp::find_subset;

fn main() {
    let result = find_subset(&vec![1, 2, 3, 4, 5, 6, 7, -8, 9, -10], -18);
    println!("{:?}", result);
}
```
Output
```
[[-8, -10]]
```
