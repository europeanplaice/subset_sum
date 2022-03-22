# Subset Sum(dpss)
This library is a rust implementation of an algorithm that solves subset sum problem. It is available for both Python and Rust.

## What is subset sum problem?

Assuming there is a list of integers (such as [1, 2, 3, 6, -9, 11]), and another integer (such as 
6), subset sum problem is the question to answer the subsets that sum to the specified integer. In this case, the answer is [1, 2, 3] and [-9, 1, 3, 11]. 

For detail information of subset sum problem, please refer to [https://en.wikipedia.org/wiki/Subset_sum_problem](https://en.wikipedia.org/wiki/Subset_sum_problem)

## What is DPSS?
[DPSS](https://github.com/europeanplaice/subset_sum) provides a tool to solve this problem without any specialized math knowledge.
It also offers a method to solve multiple subset sum problem.

## How to use DPSS?
The easiest way to use this tool is the [Google Colab Notebook](https://colab.research.google.com/github/europeanplaice/subset_sum/blob/main/python/python_subset_sum.ipynb) that I made. I also explain the other ways in [https://github.com/europeanplaice/subset_sum](https://github.com/europeanplaice/subset_sum) .

Or, there is [a WebAssenbly implementation](find_subset.html). But, it is not stable and I'm still working on it.

## What are the applications of subset sum problem and this tool?
This tool can be used in bank reconciliation.  
[Here](https://colab.research.google.com/github/europeanplaice/subset_sum/blob/main/python/bank_reconciliation.ipynb) is a Google Colab Notebook that shows the example of the usage of DPSS in bank reconciliation. Inside of this example, a function of solving multiple subset sum problem is used.
