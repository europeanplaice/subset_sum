# Subset Sum(dpss)

[![Downloads](https://static.pepy.tech/personalized-badge/dpss?period=total&units=none&left_color=grey&right_color=brightgreen&left_text=PyPI%20Downloads)](https://pepy.tech/project/dpss)
[![PyPI - Downloads](https://img.shields.io/pypi/dd/dpss?label=PyPI%20Downloads%20%28Without%20Mirrors%29)](https://pypistats.org/packages/dpss)
[![Crates.io](https://img.shields.io/crates/d/subset_sum?label=crates.io%20Downloads)](https://crates.io/crates/subset_sum)
[![Crates.io (recent)](https://img.shields.io/crates/dr/subset_sum?label=crates.io%20Downloads%20%28recent%29)](https://crates.io/crates/subset_sum)
[![GitHub all releases](https://img.shields.io/github/downloads/europeanplaice/subset_sum/total?label=GitHub%20releases%20Downloads)](https://tooomm.github.io/github-release-stats/?username=europeanplaice&repository=subset_sum)
[![GitHub Repo stars](https://img.shields.io/github/stars/europeanplaice/subset_sum?style=social)](https://github.com/europeanplaice/subset_sum)

This is a Rust implementation that calculates subset sum problem using dynamic programming. It solves subset sum problem and returns a set of decomposed integers. It also can match corresponding numbers from two vectors and be used for Account reconciliation.

There are three ways to use this program.
* [CLI](#CLI)
* [Rust](#rust)
* [Python](#python)

And it has three methods.

* `find_subset`  
    * It finds a subset from an array.
* `Sequence Matcher (One-to-Many)`
    * It finds One-to-Many relationships with two arrays.
* `Sequence Matcher (Many-to-Many)`
    * It finds Many-to-Many relationships with two arrays.

`dpss` is short for `dynamic programming subset sum`.

## Links

|Name|URL|
|--|--|
|github|https://github.com/europeanplaice/subset_sum|
|crates.io|https://crates.io/crates/subset_sum|
|docs.rs|https://docs.rs/subset_sum/latest/dpss/|
|pypi|https://pypi.org/project/dpss/|

## <a id="CLI"></a>CLI

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
[(3, [3]), (5, [5]), (7, [5, 4, -3, 1])]
[(3, [3]), (5, [4, 1]), (7, [5, 5, -3])]
[(3, [5, -3, 1]), (5, [5]), (7, [3, 4])]
```

### Sequence Matcher (Many-to-Many)

`arr1.txt`
```
1980
2980
3500
4000
1050
```

`arr2.txt`
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

Call `subset_sum.exe arr1.txt arr2.txt m2m 10 5`
In this case, 10 is `n_candidates` that is the number of candidates to be selected. A default value is 10.
5 is `max_key_length` that is the maximum length of the keys as a group. A default value is 2.

In this example, the output is   
```
[([1050], [1050]), ([1980], [30, 1950]), ([4000], [20, 3980]), ([2980], [80, 2900]), ([3500], [200, 3300])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([1050], [1050]), ([4000], [20, 3980]), ([3500], [200, 3300])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([3500], [200, 3300]), ([4000], [20, 3980]), ([1050], [1050])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([4000], [20, 3980]), ([3500], [200, 3300]), ([1050], [1050])]
[([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950]), ([4000], [20, 3980]), ([3500], [200, 3300])]
[([2980], [80, 2900]), ([4000], [20, 3980]), ([1050], [1050]), ([3500], [200, 3300]), ([1980], [30, 1950])]
[([3500], [200, 3300]), ([4000], [20, 30, 1050, 2900]), ([1050, 1980, 2980], [80, 1950, 3980])]
[([3500], [200, 3300]), ([4000], [20, 3980]), ([2980], [80, 2900]), ([1980], [30, 1950]), ([1050], [1050])]
[([4000], [20, 30, 1050, 2900]), ([3500], [200, 3300]), ([1050, 1980, 2980], [80, 1950, 3980])]
[([4000], [20, 3980]), ([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950]), ([3500], [200, 3300])]
[([4000], [20, 3980]), ([2980], [80, 2900]), ([1980], [30, 1950]), ([1050], [1050]), ([3500], [200, 3300])]
[([4000], [20, 3980]), ([3500], [200, 3300]), ([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950])]
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


print(dpss.sequence_matcher_m2m([1980, 2980, 3500, 4000, 1050], [1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10, 5))
>>>[[([1050], [1050]), ([1980], [30, 1950]), ([4000], [20, 3980]), ([2980], [80, 2900]), ([3500], [200, 3300])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([1050], [1050]), ([4000], [20, 3980]), ([3500], [200, 3300])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([3500], [200, 3300]), ([4000], [20, 3980]), ([1050], [1050])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([4000], [20, 3980]), ([3500], [200, 3300]), ([1050], [1050])]
[([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950]), ([4000], [20, 3980]), ([3500], [200, 3300])]
[([2980], [80, 2900]), ([4000], [20, 3980]), ([1050], [1050]), ([3500], [200, 3300]), ([1980], [30, 1950])]
[([3500], [200, 3300]), ([4000], [20, 30, 1050, 2900]), ([1050, 1980, 2980], [80, 1950, 3980])]
[([3500], [200, 3300]), ([4000], [20, 3980]), ([2980], [80, 2900]), ([1980], [30, 1950]), ([1050], [1050])]
[([4000], [20, 30, 1050, 2900]), ([3500], [200, 3300]), ([1050, 1980, 2980], [80, 1950, 3980])]
[([4000], [20, 3980]), ([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950]), ([3500], [200, 3300])]
[([4000], [20, 3980]), ([2980], [80, 2900]), ([1980], [30, 1950]), ([1050], [1050]), ([3500], [200, 3300])]
[([4000], [20, 3980]), ([3500], [200, 3300]), ([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950])]]
```

## <a id="rust"></a>Use in Rust

Please check https://crates.io/crates/subset_sum.

`Cargo.toml`
```
[dependencies]
dpss = { version = "(version)", package = "subset_sum" }
```

### Find subset
`main.rs`
```rust
use dpss::dp::find_subset;

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
use dpss::dp::sequence_matcher;

fn main() {
    let result = sequence_matcher(&mut vec![3, 5, 7], &mut vec![1, 5, -3, 4, 5, 3]);
    println!("{:?}", result);
}
```
Output
```
[
[(3, [3]), (5, [5]), (7, [5, 4, -3, 1])]
[(3, [3]), (5, [4, 1]), (7, [5, 5, -3])]
[(3, [5, -3, 1]), (5, [5]), (7, [3, 4])]
]
```
### Sequence Matcher (Many-to-Many)
`main.rs`
```rust
use dpss::dp::sequence_matcher_m2m;

fn main() {
    let result = sequence_matcher_m2m(&mut vec![1980, 2980, 3500, 4000, 1050], &mut vec![1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10, 5);
    println!("{:?}", result);
}
```
Output
```
[([1050], [1050]), ([1980], [30, 1950]), ([4000], [20, 3980]), ([2980], [80, 2900]), ([3500], [200, 3300])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([1050], [1050]), ([4000], [20, 3980]), ([3500], [200, 3300])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([3500], [200, 3300]), ([4000], [20, 3980]), ([1050], [1050])]
[([1980], [30, 1950]), ([2980], [80, 2900]), ([4000], [20, 3980]), ([3500], [200, 3300]), ([1050], [1050])]
[([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950]), ([4000], [20, 3980]), ([3500], [200, 3300])]
[([2980], [80, 2900]), ([4000], [20, 3980]), ([1050], [1050]), ([3500], [200, 3300]), ([1980], [30, 1950])]
[([3500], [200, 3300]), ([4000], [20, 30, 1050, 2900]), ([1050, 1980, 2980], [80, 1950, 3980])]
[([3500], [200, 3300]), ([4000], [20, 3980]), ([2980], [80, 2900]), ([1980], [30, 1950]), ([1050], [1050])]
[([4000], [20, 30, 1050, 2900]), ([3500], [200, 3300]), ([1050, 1980, 2980], [80, 1950, 3980])]
[([4000], [20, 3980]), ([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950]), ([3500], [200, 3300])]
[([4000], [20, 3980]), ([2980], [80, 2900]), ([1980], [30, 1950]), ([1050], [1050]), ([3500], [200, 3300])]
[([4000], [20, 3980]), ([3500], [200, 3300]), ([2980], [80, 2900]), ([1050], [1050]), ([1980], [30, 1950])]
```
