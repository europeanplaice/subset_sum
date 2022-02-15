# Subset Sum

This is a Rust implementation that calculates subset sum problem. It returns sets of integers that sum up to a target value.

## Installation
Binary files are provided on the [Releases](https://github.com/europeanplaice/subset_sum/releases) page. When you download one of these, please add it to your PATH manually.

## Usage

### Subset sum

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

#### Example 

Call `subset_sum.exe num_set.txt 3`  
The executable's name `subset_sum.exe` would be different from your choice. Change this example along with your environment.

In this example, the output is   
`[[1, 2], [2, -3, 4], [1, -3, 5]]`

### Sequence Matcher

`key.txt`
```
3
5
7
```

`targets.txt`
```
1
5
-3
4
5
3
```

Call `subset_sum.exe key.txt targets.txt`

In this example, the output is   
```
[([3], 3), ([5], 5), ([1, -3, 4, 5], 7)]
[([3], 3), ([1, 4], 5), ([-3, 5, 5], 7)]
[([1, -3, 5], 3), ([5], 5), ([4, 3], 7)]
```

## Use in Rust

`Cargo.toml`
```
[dependencies]
subset_sum = "(version)"
```
Example
```
subset_sum = "0.7.0"
```

### Subset sum
`main.rs`
```rust
use subset_sum::dp::find_subset;

fn main() {
    let result = sequence_matcher(&mut vec![3, 5, 7], &mut vec![1, 5, -3, 4, 5, 3]);
    println!("{:?}", result);
}
```
Output
```
[[-8, -10]]
```
### Sequence Matcher
`main.rs`
```rust
use subset_sum::dp::sequence_matcher;

fn main() {
    let result = sequence_matcher(&mut vec![3, 5, 7], &mut vec![1, 5, -3, 4, 5, 3]);
    println!("{:?}", result);
}
```
Output
```
[
 [([3], 3), ([5], 5), ([1, -3, 4, 5], 7)], 
 [([3], 3), ([1, 4], 5), ([-3, 5, 5], 7)], 
 [([1, -3, 5], 3), ([5], 5), ([4, 3], 7)]
]
```
