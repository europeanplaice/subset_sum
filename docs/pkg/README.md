# Subset Sum(dpss)

[![Downloads](https://static.pepy.tech/personalized-badge/dpss?period=total&units=none&left_color=grey&right_color=brightgreen&left_text=PyPI%20Downloads)](https://pepy.tech/project/dpss)
[![PyPI - Downloads](https://img.shields.io/pypi/dd/dpss?label=PyPI%20Downloads%20%28Without%20Mirrors%29)](https://pypistats.org/packages/dpss)
[![Crates.io](https://img.shields.io/crates/d/subset_sum?label=crates.io%20Downloads)](https://crates.io/crates/subset_sum)
[![Crates.io (recent)](https://img.shields.io/crates/dr/subset_sum?label=crates.io%20Downloads%20%28recent%29)](https://crates.io/crates/subset_sum)
[![GitHub all releases](https://img.shields.io/github/downloads/europeanplaice/subset_sum/total?label=GitHub%20releases%20Downloads)](https://tooomm.github.io/github-release-stats/?username=europeanplaice&repository=subset_sum)
[![GitHub Repo stars](https://img.shields.io/github/stars/europeanplaice/subset_sum?style=social)](https://github.com/europeanplaice/subset_sum)

This is a Rust implementation that calculates subset sum problem using dynamic programming. It solves subset sum problem and returns a set of decomposed integers. It also can match corresponding numbers from two vectors and be used for Account reconciliation.

Any feedback is welcome!

There are three ways to use this program.
* [CLI](#CLI)
* [Rust](#rust)
* [Python](#python)

Here is an out of the box example you can run now in google colab.
https://colab.research.google.com/github/europeanplaice/subset_sum/blob/main/python/python_subset_sum.ipynb

And it has three methods.

* `find_subset`  
    * It finds a subset from an array.
* `find_subset_fast_only_positive`  
    * It finds a subset from an array. It can't accept negative values but relatively faster.
* `Sequence Matcher`
    * It finds subset sum relationships with two arrays.

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

Call `subset_sum.exe num_set.txt 3 3`  
The executable's name `subset_sum.exe` would be different from your choice. Change this example along with your environment.
The second argument is the target sum.
The third argument is the maximum length of the combination.

In this example, the output is   
`[[2, 1], [4, -3, 2], [5, -3, 1]]`

### Sequence Matcher

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

Call `subset_sum.exe arr1.txt arr2.txt 100 100 10`  

In this example, the output is   
```
[([1050], [1050]), ([1980], [30, 1950]), ([2980], [80, 2900]), ([3500], [200, 3300]), ([4000], [20, 3980])]
[([1050], [1050]), ([1980], [30, 1950]), ([2980], [80, 2900]), ([3500, 4000], [20, 200, 3300, 3980])]
[([1050], [1050]), ([1980], [30, 1950]), ([3500], [200, 3300]), ([2980, 4000], [20, 80, 2900, 3980])]
...
```

## <a id="python"></a>Use in Python
### installation
```
pip install dpss
```
### Usage
#### `find_subset`
```python
import inspect
import dpss
help(dpss.find_subset)
```
```
>>> find_subset(arr, value, max_length, /)
>>>     Finds subsets sum of a target value. It can accept negative values.
>>>     # Arguments
>>>     * `arr` - An array.
>>>     * `value` - The value to the sum of the subset comes.
>>>     * `max_length` - The maximum length of combinations of the answer.
```

```python
print(dpss.find_subset([1, -2, 3, 4, 5], 2, 3))
```
```
>>> [[4, -2], [3, -2, 1]]
```
#### `find_subset_fast_only_positive`
```python
help(dpss.find_subset_fast_only_positive)
```
```
>>> find_subset_fast_only_positive(arr, value, max_length, /)
>>>    Finds subsets sum of a target value. It can't accept negative values but relatively faster.
>>>    # Arguments
>>>    * `arr` - An array.
>>>    * `value` - The value to the sum of the subset comes.
>>>    * `max_length` - The maximum length of combinations of the answer.
```
```python
print(dpss.find_subset_fast_only_positive([1, 2, 3, 4, 5], 10, 4)) 
```
```
>>> [[4, 3, 2, 1], [5, 3, 2], [5, 4, 1]]
```

#### `sequence_matcher`
```python
help(dpss.sequence_matcher)
```
```
>>> sequence_matcher(keys, targets, max_key_length, max_target_length /)
>>>     Finds the integers from two vectors that sum to the same value.
>>>     This method assumes that the two vectors have Many-to-Many relationships.
>>>     Each integer of the `keys` vector corresponds to the multiple integers of the `targets` vector.
>>>     With this method, we can find some combinations of the integers.
>>>     # Arguments
>>>     * `keys` - An array.
>>>     * `targets` - An array.
>>>     * `max_key_length` - An integer.
>>>     * `max_target_length` - An integer.
>>>     * `n_candidates` - An integer.
```
```python
print(dpss.sequence_matcher([1980, 2980, 3500, 4000, 1050], [1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10, 10, 10))
```
```
>>> [([1050], [1050]), ([1980], [30, 1950]), ([2980], [80, 2900]), ([3500], [200, 3300]), ([4000], [20, 3980])]
>>> [([1050], [1050]), ([1980], [30, 1950]), ([2980], [80, 2900]), ([3500, 4000], [20, 200, 3300, 3980])]
>>> [([1050], [1050]), ([1980], [30, 1950]), ([3500], [200, 3300]), ([2980, 4000], [20, 80, 2900, 3980])]
...
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
    let result = find_subset(&mut vec![1, 2, 3, 4, 5], 6, 3);
    println!("{:?}", result);
}
```
Output
```
[[3, 2, 1], [4, 2], [5, 1]]
```
### Sequence Matcher
`main.rs`
```rust
use dpss::dp::sequence_matcher;

fn main() {
    let result = sequence_matcher(&mut vec![1980, 2980, 3500, 4000, 1050], &mut vec![1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10, 10, 10);
    println!("{:?}", result);
}
```
Output
```
[([1050], [1050]), ([1980], [30, 1950]), ([2980], [80, 2900]), ([3500], [200, 3300]), ([4000], [20, 3980])]
[([1050], [1050]), ([1980], [30, 1950]), ([2980], [80, 2900]), ([3500, 4000], [20, 200, 3300, 3980])]
[([1050], [1050]), ([1980], [30, 1950]), ([3500], [200, 3300]), ([2980, 4000], [20, 80, 2900, 3980])]
...
```
