# Subset Sum(dpss)

[![Downloads](https://static.pepy.tech/personalized-badge/dpss?period=month&units=international_system&left_color=grey&right_color=brightgreen&left_text=PyPI%20Downloads)](https://pepy.tech/project/dpss)
![PyPI - Downloads](https://img.shields.io/pypi/dd/dpss?label=PyPI%20Downloads%20%28Without%20Mirrors%29)
![Crates.io](https://img.shields.io/crates/d/subset_sum?label=crates.io%20Downloads)
![Crates.io (recent)](https://img.shields.io/crates/dr/subset_sum?label=crates.io%20Downloads%20%28recent%29)
![GitHub all releases](https://img.shields.io/github/downloads/europeanplaice/subset_sum/total?label=GitHub%20releases%20Downloads)

This is a Rust implementation that calculates subset sum problem using dynamic programming. It solves subset sum problem and returns a set of decomposed integers. It also can match corresponding numbers from two vectors and be used for Account reconciliation.

Python implementation is also [available](#python).

`dpss` is short for `dynamic programming subset sum`.

## Links

|Name|URL|
|--|--|
|github|https://github.com/europeanplaice/subset_sum|
|crates.io|https://crates.io/crates/subset_sum|
|docs.rs|https://docs.rs/subset_sum/latest/dpss/|
|pypi|https://pypi.org/project/dpss/|


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
`[[2, 1], [4, -3, 2], [5, -3, 1]]`

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
[([3], 3), ([5], 5), ([5, 4, -3, 1], 7)]
[([3], 3), ([4, 1], 5), ([5, 5, -3], 7)]
[([5, -3, 1], 3), ([5], 5), ([3, 4], 7)]
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

## <a id="python"></a>Use in Python
### installation
```
pip install dpss
```
### Usage
```python
import dpss
print(dpss.find_subset([1, -2, 3, 4, 5], 2))
>>> [[4, -2], [3, -2, 1]]


print(dpss.find_subset_fast_only_positive([1, 2, 3, 4, 5], 10)) 
>>> [[4, 3, 2, 1], [5, 3, 2], [5, 4, 1]]


print(dpss.sequence_matcher([3, 5, 7], [1, 5, -3, 4, 5, 3]))
>>> [[([3], 3), ([5], 5), ([5, 4, -3, 1], 7)], [([3], 3), ([4, 1], 5), ([5, 5, -3], 7)], [([5, -3, 1], 3), ([5], 5), ([3, 4], 7)]]


print(dpss.sequence_matcher_m2m([1980, 2980, 3500, 4000, 1050], [1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20]))
>>>[[([20, 30, 1050, 2900], [4000]), ([200, 3300], [3500]), ([80, 1950, 3980], [1050, 1980, 2980])], [([20, 3980], [4000]), ([80, 2900], [2980]), ([30, 1950], [1980]), ([1050], [1050]), ([200, 3300], [3500])], [([20, 3980], [4000]), ([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980]), ([200, 3300], [3500])], [([20, 3980], [4000]), ([200, 3300], [3500]), ([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980])], [([30, 1950], [1980]), ([80, 2900], [2980]), ([20, 3980], [4000]), ([200, 3300], [3500]), ([1050], [1050])], [([30, 1950], [1980]), ([80, 2900], [2980]), ([200, 3300], [3500]), ([20, 3980], [4000]), ([1050], [1050])], [([30, 1950], [1980]), ([80, 2900], [2980]), ([1050], [1050]), ([20, 3980], [4000]), ([200, 3300], [3500])], [([80, 2900], [2980]), ([20, 3980], [4000]), ([1050], [1050]), ([200, 3300], [3500]), ([30, 1950], [1980])], [([80, 2900], [2980]), ([1050], [1050]), ([30, 1950], [1980]), ([20, 3980], [4000]), ([200, 3300], [3500])], [([200, 3300], [3500]), ([20, 30, 1050, 2900], [4000]), ([80, 1950, 3980], [1050, 1980, 2980])], [([200, 3300], [3500]), ([20, 3980], [4000]), ([80, 2900], [2980]), ([30, 1950], [1980]), ([1050], [1050])], [([1050], [1050]), ([30, 1950], [1980]), ([20, 3980], [4000]), ([80, 2900], [2980]), ([200, 3300], [3500])]]
```

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
    let result = find_subset(&mut vec![1, 2, 3, 4, 5], 6);
    println!("{:?}", result);
}
```
Output
```
[[3, 2, 1], [4, 2], [5, 1]]
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
[([3], 3), ([5], 5), ([5, 4, -3, 1], 7)]
[([3], 3), ([4, 1], 5), ([5, 5, -3], 7)]
[([5, -3, 1], 3), ([5], 5), ([3, 4], 7)]
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
