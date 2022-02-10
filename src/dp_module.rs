
pub mod dp {
    //! This is a module for dynamic programming.
    
    use std::collections::VecDeque;
    use std::collections::HashMap;

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
    
    fn rec(dp: &HashMap<usize, HashMap<usize, i32>>, a: &Vec<u32>, i: usize, j: usize, route: &mut VecDeque<u32>, answer: &mut Vec<VecDeque<u32>>){

        // This code is mostly copied from https://drken1215.hatenablog.com/entry/2019/12/17/190300
        
        if i == 0 {
            if j == 0 {
                answer.push(route.clone());
            }
            return;
        }
        if  dp[&(i-1)].get(&j) != None && dp[&(i-1)][&j] != 0{
            rec(dp, a, i-1, j, route, answer);
        }
    
        if j as i32 - a[i-1] as i32 >= 0 && dp[&(i-1)].get(&(j-a[i-1] as usize)) != None && dp[&(i-1)][&(j-a[i-1] as usize)] != 0 {
            route.push_front(a[i-1]);
            rec(dp, a, i-1, j-a[i-1] as usize, route, answer);
            route.pop_front();
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
        // let mut dp: Vec<Vec<i32>> = vec![vec![0; n+1]; a.len()+1];
        let mut dp = HashMap::new();
        let length = a.len();
        for i in 0..length+1 {
            let mut dp2 = HashMap::new();
            dp2.insert(0, 0);
            dp.insert(i, dp2);
        }
        let mut dp3 = HashMap::new();
        dp3.insert(0, 1);
        *dp.get_mut(&0).unwrap() = dp3;
        for i in 0..length {
            for j in 0..n+1 {

                let v = dp.get(&i).unwrap().get(&j);
                if v == None{
                    if v != None && v.unwrap() > &0 {
                        dp.get_mut(&(i+1)).unwrap().insert(j, 0);
                        *dp.get_mut(&(i+1)).unwrap().get_mut(&j).unwrap() = *v.unwrap();
                    }
                } else {
                    if v != None && v.unwrap() > &0 {
                        *dp.get_mut(&(i+1)).unwrap().get_mut(&j).unwrap() += v.unwrap();
                    }
                }

                if j as i32 - i as i32 > n as i32 - (length as i32 - 2){
                    continue;
                }
                
                if j as u32 + a[i] < n as u32 + 1 {
                    if dp[&i].get(&j) != None && dp[&i][&j] > 0{
                        if dp[&(i+1)].get(&(j+a[i] as usize)) == None {
                            dp.get_mut(&(i+1)).unwrap().insert(j+a[i] as usize, 0);
                        }
                        *dp.get_mut(&(i+1)).unwrap().get_mut(&(j+a[i] as usize)).unwrap() += dp[&i][&j];
                    };
                }
            }   
        }
        let a_length: usize = a.len();
        let mut route: VecDeque<u32> = VecDeque::new();
        let mut answer: Vec<VecDeque<u32>> = Vec::new();
        rec(&dp, &a, a_length, n, &mut route, &mut answer);
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
        let result = dp::find_subset(&vec![1, 2, 3, 4, 5], 10);
        let route1: VecDeque<i32> = VecDeque::from(vec![2, 3, 5]);
        let route2: VecDeque<i32> = VecDeque::from(vec![1, 4, 5]);
        let route3: VecDeque<i32> = VecDeque::from(vec![1, 2, 3, 4]);
        let answer: Vec<VecDeque<i32>> = vec![route1, route2, route3];
        assert_eq!(result, answer);
        
        let result = dp::find_subset(&vec![1, 2, 3], 3);
        let route1: VecDeque<i32> = VecDeque::from(vec![3]);
        let route2: VecDeque<i32> = VecDeque::from(vec![1, 2]);
        let answer: Vec<VecDeque<i32>> = vec![route1, route2];
        assert_eq!(result, answer);


        let mut a = vec![75, 467, 512, -835, 770, -69, 10];
        a.sort();
        let result = dp::find_subset(&a, 711);
        let route1: VecDeque<i32> = VecDeque::from(vec![-69, 10, 770]);
        let answer: Vec<VecDeque<i32>> = vec![route1];
        assert_eq!(result, answer);

        let mut a = vec![ -3,  10,  56, -33,  65,  -9,   8,  72,  63,  35];
        a.sort();
        let result = dp::find_subset(&a, 7);
        let route1: VecDeque<i32> = VecDeque::from(vec![-3, 10]);
        let route2: VecDeque<i32> = VecDeque::from(vec![-33, -3, 8, 35]);
        let answer: Vec<VecDeque<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let mut a = vec![73209,95597,84735,40496,83553,95595,-628,201,27597,7904,98445,6241,33002,-776,-711,45552,86746,84248,66278,37475];
        a.sort();
        let result = dp::find_subset(&a, 72782);
        let route1: VecDeque<i32> = VecDeque::from(vec![-628, 201, 73209]);
        let answer: Vec<VecDeque<i32>> = vec![route1];
        assert_eq!(result, answer);
    }
}