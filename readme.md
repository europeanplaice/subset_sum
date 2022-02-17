# Subset Sum

This is a Rust implementation that calculates subset sum problem. It solves subset sum problem and returns a set of decomposed integers. It also can match corresponding numbers from two vectors and be used for Account reconciliation.

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

### Sequence Matcher (One-to-Many)

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

### Sequence Matcher (Many-to-Many)

`key.txt`
```
1980
2980
3500
4000
1050
```

`targets.txt`
```
1950
2900
30
80
3300
200
3980
1050
20
```

Call `subset_sum.exe key.txt targets.txt m2m`

In this example, the output is   
```
[([20, 30, 1050, 2900], [4000]), ([200, 3300], [3500]), ([80, 1950, 3980], [1050, 1980, 2980])]
[([20, 3980], [4000]), ([80, 2900], [2980]), ([30, 1950], [1980]), ([1050], [1050]), ([200, 3300], [3500])]
[([20, 3980], [4000]), ([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980]), ([200, 3300], [3500])]
[([20, 3980], [4000]), ([200, 3300], [3500]), ([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980])]
[([30, 1950], [1980]), ([80, 2900], [2980]), ([20, 3980], [4000]), ([200, 3300], [3500]), ([1050], [1050])]
[([30, 1950], [1980]), ([80, 2900], [2980]), ([200, 3300], [3500]), ([20, 3980], [4000]), ([1050], [1050])]
[([30, 1950], [1980]), ([80, 2900], [2980]), ([1050], [1050]), ([20, 3980], [4000]), ([200, 3300], [3500])]
[([80, 2900], [2980]), ([20, 3980], [4000]), ([1050], [1050]), ([200, 3300], [3500]), ([30, 1950], [1980])]
[([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980]), ([20, 3980], [4000]), ([200, 3300], [3500])]
[([200, 3300], [3500]), ([20, 30, 1050, 2900], [4000]), ([80, 1950, 3980], [1050, 1980, 2980])]
[([200, 3300], [3500]), ([20, 3980], [4000]), ([80, 2900], [2980]), ([30, 1950], [1980]), ([1050], [1050])]
[([1050], [1050]), ([30, 1950], [1980]), ([20, 3980], [4000]), ([80, 2900], [2980]), ([200, 3300], [3500])]
```
There are a lot of combinations!
## Use in Rust

`Cargo.toml`
```
[dependencies]
subset_sum = "(version)"
```
Example
```
subset_sum = "0.8.0"
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
### Sequence Matcher (One-to-Many)
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
### Sequence Matcher (Many-to-Many)
`main.rs`
```rust
use subset_sum::dp::sequence_matcher_m2m;

fn main() {
    let result = sequence_matcher_m2m(&mut vec![1980, 2980, 3500, 4000, 1050], &mut vec![1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10);
    println!("{:?}", result);
}
```
Output
```
[([20, 30, 1050, 2900], [4000]), ([200, 3300], [3500]), ([80, 1950, 3980], [1050, 1980, 2980])]
[([20, 3980], [4000]), ([80, 2900], [2980]), ([30, 1950], [1980]), ([1050], [1050]), ([200, 3300], [3500])]
[([20, 3980], [4000]), ([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980]), ([200, 3300], [3500])]
[([20, 3980], [4000]), ([200, 3300], [3500]), ([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980])]
[([30, 1950], [1980]), ([80, 2900], [2980]), ([20, 3980], [4000]), ([200, 3300], [3500]), ([1050], [1050])]
[([30, 1950], [1980]), ([80, 2900], [2980]), ([200, 3300], [3500]), ([20, 3980], [4000]), ([1050], [1050])]
[([30, 1950], [1980]), ([80, 2900], [2980]), ([1050], [1050]), ([20, 3980], [4000]), ([200, 3300], [3500])]
[([80, 2900], [2980]), ([20, 3980], [4000]), ([1050], [1050]), ([200, 3300], [3500]), ([30, 1950], [1980])]
[([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980]), ([20, 3980], [4000]), ([200, 3300], [3500])]
[([200, 3300], [3500]), ([20, 30, 1050, 2900], [4000]), ([80, 1950, 3980], [1050, 1980, 2980])]
[([200, 3300], [3500]), ([20, 3980], [4000]), ([80, 2900], [2980]), ([30, 1950], [1980]), ([1050], [1050])]
[([1050], [1050]), ([30, 1950], [1980]), ([20, 3980], [4000]), ([80, 2900], [2980]), ([200, 3300], [3500])]
```
