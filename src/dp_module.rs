
pub mod dp {
    //! This is a module for dynamic programming.
    
    use std::collections::VecDeque;
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
    pub fn find_subset(a: &Vec<i32>, n: usize) -> Vec<VecDeque<i32>>{
    
        // https://stackoverflow.com/questions/43078142/subset-sum-with-negative-values-in-c-or-c
    
        let offset: u32 = (a.iter().min().unwrap().abs() + 1) as u32;
        let mut b: Vec<u32> = Vec::new();
        for i in a{
            b.push((i + offset as i32) as u32);
        }
        let mut answer: Vec<VecDeque<i32>> = Vec::new();
        for i in 1..a.len(){
            let result = find_subset_fast_only_positive(&b, (n + i * offset as usize) as usize);
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
        
        if i == 0 {
            if j == 0 {
                answer.push(route.clone());
            }
            return;
        }
        if j as u32 >= *a_min || j as u32 == 0 {
            if dp[i-1][j] != 0 {
                rec(dp, a, i-1, j, route, answer, a_min);
            }
        
            if j as i32 - a[i-1] as i32 >= 0 && dp[i-1][j-a[i-1] as usize] != 0 {
                route.push_front(a[i-1]);
                rec(dp, a, i-1, j-a[i-1] as usize, route, answer, a_min);
                route.pop_front();
            }
        }
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
        let mut a_min = a.iter().max().unwrap();
        for i in a{
            if i > &0 {
                if a_min > &i {
                    a_min = &i
                }
            }
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n+1]; a.len()+1];
        dp[0][0] = 1;
        for i in 0..a.len() {
            for j in 0..n+1 {
                if j as u32 >= *a_min || j as u32 == 0 {
                    dp[i+1][j] += dp[i][j];
                    if j as u32 + a[i] < n as u32 + 1 {
                        dp[i+1][j+a[i] as usize] += dp[i][j];
                    }
                }
            }   
        }
        let a_length: usize = a.len();
    
        let mut route: VecDeque<u32> = VecDeque::new();
        let mut answer: Vec<VecDeque<u32>> = Vec::new();

        rec(&dp, &a, a_length, n, &mut route, &mut answer, a_min);
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
    }
}