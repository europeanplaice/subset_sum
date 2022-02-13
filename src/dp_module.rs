
pub mod dp {
    //! This is a module for dynamic programming.
    
    use std::collections::VecDeque;

    fn gcd_multi(v: Vec<u32>) -> u32 {
        let mut result = v[0];
        for i in 1..v.len() {
            result = gcd(result, v[i]);
        }
        result
    }

    fn gcd(a: u32, b: u32) -> u32{
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    
    #[test]
    
    fn test_gcd() {
        assert_eq!(gcd(20, 10), 10);
        assert_eq!(gcd(55, 5), 5);
        assert_eq!(gcd(991, 997), 1);
    }

    #[test]
    fn test_gcd_multi() {
        assert_eq!(gcd_multi(vec![5, 10, 20]), 5);
        assert_eq!(gcd_multi(vec![131, 863, 887]), 1);
    }
    /// Finds subsets sum of a target value. It can accept negative values.
    /// 
    /// # Example
    /// ```
    /// use std::collections::VecDeque;
    /// use subset_sum::dp::find_subset;
    /// let a = vec![-1, -3, -2, 6, 12, 48];
    /// let result = find_subset(&a, 0);
    /// let route1: VecDeque<i32> = VecDeque::from(vec![-1, -3, -2, 6]);
    /// let answer: Vec<VecDeque<i32>> = vec![route1];
    /// assert_eq!(result, answer);
    /// ```
    /// 
    /// # Return Value
    /// ```
    /// use std::collections::VecDeque;
    /// use subset_sum::dp::find_subset;
    /// let result = find_subset(&vec![1, 2, 3, -4, 5], 1);
    /// println!("{:?}", result);
    /// ```
    /// output: `[[1], [-3, 4]]`
    pub fn find_subset(a: &Vec<i32>, n: i32) -> Vec<VecDeque<i32>>{

        use std::cmp::min;
        use std::cmp::max;
    
        // https://stackoverflow.com/questions/43078142/subset-sum-with-negative-values-in-c-or-c
    
        // Find a subset even if an array contains negative values.
        let offset: u32 = (max(a.iter().min().unwrap().abs() + 1, min(n, 0).abs() + 1)) as u32;
        let mut b: Vec<u32> = Vec::new();
        for i in a{
            b.push((i + offset as i32) as u32);
        }
        let mut answer: Vec<VecDeque<i32>> = Vec::new();

        // We will transform the array into a new array whose elements are all positive.
        // And check if the transformed sum of the result of the new array is equal to the target value.
        // If we find the sum is the same as the target, we will return the result.
        for i in 1..a.len()+1{
            let result = find_subset_fast_only_positive(&b, (n + i as i32 * offset as i32) as usize);
            for res in result{
                let mut tempsum: i32 = 0;
                let mut new_res: VecDeque<i32> = VecDeque::new();
                for el in res{
                    tempsum += el as i32 - offset as i32;
                    new_res.push_back(el as i32 - offset as i32);
                }
                if tempsum == n as i32{
                    answer.push(new_res);
                }
            }
        }
        answer
    }
    
    fn rec(dp: &Vec<Vec<i32>>, a: &Vec<u32>, i: usize, j: usize, route: &mut VecDeque<u32>, answer: &mut Vec<VecDeque<u32>>, a_min: &u32){

        // This code is mostly copied from https://drken1215.hatenablog.com/entry/2019/12/17/190300
        
        // We follow the dp table backward to find combinations of subsets.
        // We call this function recursively twice and this means the call stack expands like a tree. 
        if i == 0 {
            if j == 0 {
                // Only if we reach the root of the dp table, we choose the combination as an answer.
                answer.push(route.clone());
            }
            return;
        }

        if dp[i-1][j] != 0 {
            rec(dp, a, i-1, j, route, answer, a_min);
        }
    
        if j as i32 - a[i-1] as i32 >= 0 && dp[i-1][j-a[i-1] as usize] != 0 {
            // Choose this element as a candidate for an answer.
            route.push_front(a[i-1]);
            rec(dp, a, i-1, j-a[i-1] as usize, route, answer, a_min);
            // Remove this element after we reach i == 0 regardless of whether we reach j == 0.
            route.pop_front();
        }
    }

    pub fn filter_j_idx(n: usize, a:  &Vec<u32>) -> (Vec<usize>, u32){
        // a_min is the minimum number in an except for zero.
        let mut a_min = a.iter().max().unwrap();
        let mut a_no_zero: Vec<u32> = vec![];
        for i in a{
            if i > &0 {
                if a_min > &i {
                    a_min = &i
                }
                a_no_zero.push(*i);
            }
        }
        let mut j_indexes: Vec<usize> = vec![];
        let gcd = gcd_multi(a_no_zero);
        // j of the range of 1 to a_min-1 must be zero.
        // For example, if a_min = 10, there is no way to make sum 5.
        // Also, if j == 8 and target = 10 and a_min=5, we can't reach 10.
        // If all the numbers are even, j should be even.
        for j in 0..n+1{
            if (j as u32 >= *a_min && j as u32 <=  n as u32 - *a_min && j as u32 % gcd == 0) || j as u32 == 0 || j == n  {
                j_indexes.push(j)
            }
        }
        (j_indexes, *a_min)
    }

    #[test]
    fn test_filter_j_idx(){
        let (result, _a_min) = filter_j_idx(10, &vec![3, 4, 5, 6, 7 ,8, 9, 10]);
        let answer: Vec<usize> = vec![0, 3, 4, 5, 6, 7, 10];
        assert_eq!(result, answer);

        let (result, _a_min) = filter_j_idx(5, &vec![3, 4, 5]);
        let answer: Vec<usize> = vec![0, 5];
        assert_eq!(result, answer);

        let (result, _a_min) = filter_j_idx(10, &vec![0, 2, 4, 6, 8]);
        let answer: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
        assert_eq!(result, answer);

        let (result, _a_min) = filter_j_idx(20, &vec![10, 20, 30, 40, 50]);
        let answer: Vec<usize> = vec![0, 10, 20];
        assert_eq!(result, answer);

        let (result, _a_min) = filter_j_idx(8, &vec![2, 3, 5, 7]);
        let answer: Vec<usize> = vec![0, 2, 3, 4, 5, 6, 8];
        assert_eq!(result, answer);
    }
    
    /// Finds subsets sum of a target value. It can't accept negative values but relatively faster.
    /// 
    /// # Example
    /// ```
    /// use std::collections::VecDeque;
    /// use subset_sum::dp::find_subset_fast_only_positive;
    /// let result = find_subset_fast_only_positive(&vec![1, 2, 3], 3);
    /// let route1: VecDeque<u32> = VecDeque::from(vec![1, 2]);
    /// let route2: VecDeque<u32> = VecDeque::from(vec![3]);
    /// let answer: Vec<VecDeque<u32>> = vec![route1, route2];
    /// assert_eq!(result, answer);
    /// ```
    /// # Return Value
    /// ```
    /// use std::collections::VecDeque;
    /// use subset_sum::dp::find_subset_fast_only_positive;
    /// let result = find_subset_fast_only_positive(&vec![1, 2, 3, 4, 5], 10);
    /// println!("{:?}", result);
    /// ```
    /// output: `[[2, 3, 5], [1, 4, 5], [1, 2, 3, 4]]`
    pub fn find_subset_fast_only_positive(a: &Vec<u32>, n: usize) ->  Vec<VecDeque<u32>>
    {

        // dp is a table that stores the information of subset sum.
        // dp[i][j] is the number of ways to make sum j with i element.
        // We follow from the start of this table.
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n+1]; a.len()+1];
        dp[0][0] = 1;

        let (j_indexes, a_min) = filter_j_idx(n, a);
        for i in 0..a.len() {
            for j in &j_indexes{

                // If we don't choose to select an element to sum, 
                // the ways to make a sum are the same as with the previous element.
                dp[i+1][*j] += dp[i][*j];

                // Skip if j + the element is larger than the target value.
                if *j as u32 + a[i] < n as u32 + 1 {
                    // This means we find another way to make sum j with i elements
                    // when we choose this element as an element to sum.
                    dp[i+1][j+a[i] as usize] += dp[i][*j];
                }
            }   
        }
        let a_length: usize = a.len();
    
        let mut route: VecDeque<u32> = VecDeque::new();
        let mut answer: Vec<VecDeque<u32>> = Vec::new();

        rec(&dp, &a, a_length, n, &mut route, &mut answer, &a_min);
        answer
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;



    #[test]
    fn test_find_subset_fast_only_positive(){

        let result = dp::find_subset_fast_only_positive(&vec![1, 2, 3], 3);
        let route1: VecDeque<u32> = VecDeque::from(vec![1, 2]);
        let route2: VecDeque<u32> = VecDeque::from(vec![3]);
        let answer: Vec<VecDeque<u32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset_fast_only_positive(&vec![0, 3, 5, 10], 3);
        let route1: VecDeque<u32> = VecDeque::from(vec![3]);
        let route2: VecDeque<u32> = VecDeque::from(vec![0, 3]);
        let answer: Vec<VecDeque<u32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset_fast_only_positive(&vec![1, 2, 3, 0], 3);
        let route1: VecDeque<u32> = VecDeque::from(vec![1, 2]);
        let route2: VecDeque<u32> = VecDeque::from(vec![3]);
        let route3: VecDeque<u32> = VecDeque::from(vec![1, 2, 0]);
        let route4: VecDeque<u32> = VecDeque::from(vec![3, 0]);
        let answer: Vec<VecDeque<u32>> = vec![route1, route2, route3, route4];
        assert_eq!(result, answer);
    }

    #[test]
    fn test_find_test(){
        let result = dp::find_subset(&vec![1, 2, 3], 3);
        let route1: VecDeque<i32> = VecDeque::from(vec![3]);
        let route2: VecDeque<i32> = VecDeque::from(vec![1, 2]);
        let answer: Vec<VecDeque<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset(&vec![1, 2, 3, 4, 5], 10);
        let route1: VecDeque<i32> = VecDeque::from(vec![2, 3, 5]);
        let route2: VecDeque<i32> = VecDeque::from(vec![1, 4, 5]);
        let route3: VecDeque<i32> = VecDeque::from(vec![1, 2, 3, 4]);
        let answer: Vec<VecDeque<i32>> = vec![route1, route2, route3];
        assert_eq!(result, answer);

        let a = vec![75, 467, 512, -835, 770, -69, 10];
        let result = dp::find_subset(&a, 711);
        let route1: VecDeque<i32> = VecDeque::from(vec![770, -69, 10]);
        let answer: Vec<VecDeque<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![ -3,  10,  56, -33,  65,  -9,   8,  72,  63,  35];
        let result = dp::find_subset(&a, 7);
        let route1: VecDeque<i32> = VecDeque::from(vec![-3, 10]);
        let route2: VecDeque<i32> = VecDeque::from(vec![-3, -33, 8, 35]);
        let answer: Vec<VecDeque<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let a = vec![73209,95597,84735,40496,83553,95595,-628,201,27597,7904,98445,6241,33002,-776,-711,45552,86746,84248,66278,37475];
        let result = dp::find_subset(&a, 72782);
        let route1: VecDeque<i32> = VecDeque::from(vec![73209, -628, 201]);
        let answer: Vec<VecDeque<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![ -1, 2, 3];
        let result = dp::find_subset(&a, -1);
        let route1: VecDeque<i32> = VecDeque::from(vec![-1]);
        let answer: Vec<VecDeque<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![ -10, 5, -2];
        let result = dp::find_subset(&a, -5);
        let route1: VecDeque<i32> = VecDeque::from(vec![-10, 5]);
        let answer: Vec<VecDeque<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![-3, -5, -7];
        let result = dp::find_subset(&a, -15);
        let route1: VecDeque<i32> = VecDeque::from(vec![-3, -5, -7]);
        let answer: Vec<VecDeque<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![-100, 10, 20];
        let result = dp::find_subset(&a, -70);
        let route1: VecDeque<i32> = VecDeque::from(vec![-100, 10, 20]);
        let answer: Vec<VecDeque<i32>> = vec![route1];
        assert_eq!(result, answer);
    }
}