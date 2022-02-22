pub mod dp {
    //! This is a module for dynamic programming.

    fn gcd_multi(v: Vec<u32>) -> u32 {
        let mut result = v[0];
        for i in 1..v.len() {
            result = gcd(result, v[i]);
        }
        result
    }

    fn gcd(a: u32, b: u32) -> u32 {
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
    ///
    /// use subset_sum::dp::find_subset;
    /// let a = vec![-1, -3, -2, 6, 12, 48];
    /// let result = find_subset(&a, 0);
    /// let route1: Vec<i32> = vec![6, -2, -3, -1];
    /// let answer: Vec<Vec<i32>> = vec![route1];
    /// assert_eq!(result, answer);
    /// ```
    ///
    /// # Return Value
    /// ```
    ///
    /// use subset_sum::dp::find_subset;
    /// let result = find_subset(&vec![1, 2, 3, -4, 5], 1);
    /// println!("{:?}", result);
    /// ```
    /// output: `[[1], [4, -3]]`
    pub fn find_subset(a: &Vec<i32>, n: i32) -> Vec<Vec<i32>> {
        use std::cmp::max;
        use std::cmp::min;
        // https://stackoverflow.com/questions/43078142/subset-sum-with-negative-values-in-c-or-c
        // Find a subset even if an array contains negative values.
        let offset: u32 = (max(a.iter().min().unwrap().abs() + 1, min(n, 0).abs() + 1)) as u32;
        let mut b: Vec<u32> = Vec::new();
        for i in a {
            b.push((i + offset as i32) as u32);
        }
        let mut answer: Vec<Vec<i32>> = Vec::new();

        // We will transform the array into a new array whose elements are all positive.
        // And check if the transformed sum of the result of the new array is equal to the target value.
        // If we find the sum is the same as the target, we will return the result.
        for i in 1..a.len() + 1 {
            let result =
                find_subset_fast_only_positive(&b, (n + i as i32 * offset as i32) as usize);
            for res in result {
                let mut tempsum: i32 = 0;
                let mut new_res: Vec<i32> = Vec::new();
                for el in res {
                    tempsum += el as i32 - offset as i32;
                    new_res.push(el as i32 - offset as i32);
                }
                if tempsum == n as i32 {
                    answer.push(new_res);
                }
            }
        }
        answer
    }

    fn rec(
        dp: &Vec<Vec<i32>>,
        a: &Vec<u32>,
        i: usize,
        j: usize,
        route: &mut Vec<u32>,
        answer: &mut Vec<Vec<u32>>,
        a_min: &u32,
    ) {
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

        if dp[i - 1][j] != 0 {
            rec(dp, a, i - 1, j, route, answer, a_min);
        }

        if j as i32 - a[i - 1] as i32 >= 0 && dp[i - 1][j - a[i - 1] as usize] != 0 {
            // Choose this element as a candidate for an answer.
            route.push(a[i - 1]);
            rec(dp, a, i - 1, j - a[i - 1] as usize, route, answer, a_min);
            // Remove this element after we reach i == 0 regardless of whether we reach j == 0.
            route.pop();
        }
    }

    fn filter_j_idx(n: usize, a: &Vec<u32>) -> (Vec<usize>, u32) {
        // a_min is the minimum number in an except for zero.
        let mut a_min = a.iter().max().unwrap();
        let mut a_no_zero: Vec<u32> = vec![];
        for i in a {
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
        for j in 0..n + 1 {
            if (j as u32 >= *a_min && j as u32 <= n as u32 - *a_min && j as u32 % gcd == 0)
                || j as u32 == 0
                || j == n
            {
                j_indexes.push(j)
            }
        }
        (j_indexes, *a_min)
    }

    #[test]
    fn test_filter_j_idx() {
        let (result, _a_min) = filter_j_idx(10, &vec![3, 4, 5, 6, 7, 8, 9, 10]);
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
    ///
    /// use subset_sum::dp::find_subset_fast_only_positive;
    /// let result = find_subset_fast_only_positive(&vec![1, 2, 3], 3);
    /// let route1: Vec<u32> = vec![2, 1];
    /// let route2: Vec<u32> = vec![3];
    /// let answer: Vec<Vec<u32>> = vec![route1, route2];
    /// assert_eq!(result, answer);
    /// ```
    /// # Return Value
    /// ```
    ///
    /// use subset_sum::dp::find_subset_fast_only_positive;
    /// let result = find_subset_fast_only_positive(&vec![1, 2, 3, 4, 5], 10);
    /// println!("{:?}", result);
    /// ```
    /// output: `[[5, 3, 2], [5, 4, 1], [4, 3, 2, 1]]`
    pub fn find_subset_fast_only_positive(a: &Vec<u32>, n: usize) -> Vec<Vec<u32>> {
        // dp is a table that stores the information of subset sum.
        // dp[i][j] is the number of ways to make sum j with i element.
        // We follow from the start of this table.
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; a.len() + 1];
        dp[0][0] = 1;

        let (j_indexes, a_min) = filter_j_idx(n, a);
        for i in 0..a.len() {
            for j in &j_indexes {
                // If we don't choose to select an element to sum,
                // the ways to make a sum are the same as with the previous element.
                dp[i + 1][*j] += dp[i][*j];

                // Skip if j + the element is larger than the target value.
                if *j as u32 + a[i] < n as u32 + 1 {
                    // This means we find another way to make sum j with i elements
                    // when we choose this element as an element to sum.
                    dp[i + 1][j + a[i] as usize] += dp[i][*j];
                }
            }
        }
        let a_length: usize = a.len();
        let mut route: Vec<u32> = Vec::new();
        let mut answer: Vec<Vec<u32>> = Vec::new();

        rec(&dp, &a, a_length, n, &mut route, &mut answer, &a_min);
        answer
    }

    fn vec_remove(a: &mut Vec<i32>, v: i32) {
        let index = a.iter().position(|x| *x == v).unwrap();
        a.remove(index);
    }

    /// Finds the integers from two vectors that sum to the same value.
    /// This method assumes that the two vectors have One-to-Many relationships.
    /// Each integer of the `key` vector corresponds to the multiple integers of the `value` vector.
    /// # Example
    /// ```
    ///
    /// use subset_sum::dp::sequence_matcher;
    /// let answer = sequence_matcher(&mut vec![3, 5, 7], &mut vec![1, 5, -3, 4, 5, 3]);
    /// assert_eq!(answer, vec![
    ///     vec![
    ///         (vec![3], 3),
    ///         (vec![5], 5),
    ///         (vec![5, 4, -3, 1], 7),
    ///      ],
    ///     vec![
    ///         (vec![3], 3),
    ///         (vec![4, 1], 5),
    ///         (vec![5, 5, -3], 7),
    ///      ],
    ///     vec![
    ///         (vec![5, -3, 1], 3),
    ///         (vec![5], 5),
    ///         (vec![3, 4], 7),
    ///      ],
    /// ]);
    ///
    /// let answer_unchanged: Vec<Vec<(Vec<i32>, i32)>> = Vec::new();
    /// let answer = sequence_matcher(&mut vec![10, 20], &mut vec![9, 21]);
    /// assert_eq!(answer, answer_unchanged);
    /// ```
    pub fn sequence_matcher(
        key: &mut Vec<i32>,
        targets: &mut Vec<i32>,
    ) -> Vec<Vec<(Vec<i32>, i32)>> {
        let mut group: Vec<(Vec<i32>, i32)> = Vec::new();
        let mut answer: Vec<Vec<(Vec<i32>, i32)>> = Vec::new();
        sequence_matcher_core(key, targets, &mut group, &mut answer);
        answer
    }

    fn sequence_matcher_core(
        key: &mut Vec<i32>,
        targets: &mut Vec<i32>,
        group: &mut Vec<(Vec<i32>, i32)>,
        answer: &mut Vec<Vec<(Vec<i32>, i32)>>,
    ) {
        if key.len() == 0 && targets.len() == 0 {
            answer.push(group.clone());
            return;
        }
        if key.len() == 0 && targets.len() > 0 {
            return;
        }
        if key.len() > 0 && targets.len() == 0 {
            return;
        }

        let set_: Vec<Vec<i32>> = find_subset(&targets, key[0]);
        for set in set_ {
            group.push((set.clone(), key[0]));
            let i2 = key[0].clone();
            for el in set.clone() {
                vec_remove(targets, el);
            }
            vec_remove(key, key[0]);

            sequence_matcher_core(key, targets, group, answer);
            group.pop();
            for el in set.clone() {
                targets.push(el);
            }
            key.push(i2);
        }
    }

    /// Finds the integers from two vectors that sum to the same value.
    /// This method assumes that the two vectors have Many-to-Many relationships.
    /// Each integer of the `key` vector corresponds to the multiple integers of the `value` vector.
    /// With this method, we can find multiple combinations of the integers.
    /// # Example
    ///
    /// ```rust
    ///
    ///use subset_sum::dp::sequence_matcher_m2m;
    ///let answer = sequence_matcher_m2m(&mut vec![1980, 2980, 3500, 4000, 1050], &mut vec![1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10);
    ///assert_eq!(answer[0], vec![
    ///    (vec![20, 30, 1050, 2900],
    ///     vec![4000]),
    ///
    ///     (vec![200, 3300],
    ///     vec![3500]),
    ///
    ///     (vec![80, 1950, 3980],
    ///     vec![1050, 1980, 2980]),
    ///
    ///
    ///    ]);
    ///assert_eq!(answer[1], vec![
    ///    (vec![20, 3980],
    ///     vec![4000]),
    ///
    ///     (vec![80, 2900],
    ///     vec![2980]),
    ///
    ///     (vec![30, 1950],
    ///     vec![1980]),
    ///
    ///     (vec![1050],
    ///     vec![1050]),
    ///
    ///     (vec![200, 3300],
    ///     vec![3500]),
    ///
    ///    ]);
    /// ```
    pub fn sequence_matcher_m2m(
        key: &mut Vec<i32>,
        targets: &mut Vec<i32>,
        n_max: usize,
    ) -> Vec<Vec<(Vec<i32>, Vec<i32>)>> {
        use rand::seq::SliceRandom;

        let mut group: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
        let mut answer: Vec<Vec<(Vec<i32>, Vec<i32>)>> = Vec::new();
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed([13; 32]);
        if key.iter().sum::<i32>() != targets.iter().sum() {
            println!("The sum of the key must be equal to the sum of the targets.");
            return answer;
        }
        for _i in 0..n_max {
            sequence_matcher_core_m2m(key, targets, &mut group, &mut answer, 1, &mut key.clone());
            key.shuffle(&mut rng);
        }
        answer.sort();
        answer.dedup();
        answer
    }

    fn sequence_matcher_core_m2m(
        key: &mut Vec<i32>,
        targets: &mut Vec<i32>,
        group: &mut Vec<(Vec<i32>, Vec<i32>)>,
        answer: &mut Vec<Vec<(Vec<i32>, Vec<i32>)>>,
        n_key: usize,
        key_orig: &mut Vec<i32>,
    ) {
        if key.iter().sum::<i32>() != targets.iter().sum() {
            return;
        }

        if key.len() == 0 && targets.len() == 0 {
            answer.push(group.clone());
            return;
        }
        if (key.len() == 0 && targets.len() > 0) || (key.len() > 0 && targets.len() == 0) {
            sequence_matcher_core_m2m(key, targets, group, answer, n_key + 1, key_orig);
        }

        if n_key > key.len() {
            return;
        }

        let mut sum_key = 0;
        let mut vec_key = vec![];
        for i in 0..n_key {
            sum_key += key[i];
            vec_key.push(key[i].clone())
        }
        let set_: Vec<Vec<i32>> = find_subset(&targets, sum_key);
        if set_.len() == 0 {
            sequence_matcher_core_m2m(key, targets, group, answer, n_key + 1, key_orig);
        }
        for set in set_ {
            let mut _set = Vec::from(set.clone());
            _set.sort();
            let mut _vec_key = vec_key.clone();
            _vec_key.sort();
            group.push((_set, _vec_key.clone()));
            for el in set.clone() {
                vec_remove(targets, el);
            }
            for i in vec_key.clone() {
                vec_remove(key, i);
            }
            sequence_matcher_core_m2m(key, targets, group, answer, n_key, key_orig);
            group.pop();
            for el in set.clone() {
                targets.push(el);
            }
            for i in vec_key.clone() {
                key.push(i);
            }
        }
    }

    #[test]
    fn test_sequence_matcher() {
        let answer = sequence_matcher(&mut vec![1, 6], &mut vec![1, 3, 3]);
        assert_eq!(answer, vec![vec![(vec![1], 1), (vec![3, 3], 6)]]);

        let answer = sequence_matcher(&mut vec![10, 20], &mut vec![-10, 20, 16, 4]);
        // let answer = transpose(answer);
        assert_eq!(answer, vec![vec![(vec![20, -10], 10), (vec![4, 16], 20),],]);

        let answer_unchanged: Vec<Vec<(Vec<i32>, i32)>> = Vec::new();

        let answer = sequence_matcher(&mut vec![10, 20], &mut vec![10, 21]);
        assert_eq!(answer, answer_unchanged);

        let answer = sequence_matcher(&mut vec![3, 5, 7], &mut vec![1, 5, -3, 4, 5, 3]);
        assert_eq!(
            answer,
            vec![
                vec![(vec![3], 3), (vec![5], 5), (vec![5, 4, -3, 1], 7),],
                vec![(vec![3], 3), (vec![4, 1], 5), (vec![5, 5, -3], 7),],
                vec![(vec![5, -3, 1], 3), (vec![5], 5), (vec![3, 4], 7),],
            ]
        );
    }

    #[test]
    fn test_sequence_matcher_m2m() {
        let answer =
            sequence_matcher_m2m(&mut vec![1, 2, 3, 4, 5], &mut vec![11, -8, 14, -7, 5], 10);
        assert_eq!(
            answer[0],
            vec![(vec![-8, -7, 5, 11], vec![1]), (vec![14], vec![2, 3, 4, 5]),]
        );

        let answer = sequence_matcher_m2m(
            &mut vec![1000, 1100, 150, 123, 5, 10],
            &mut vec![2100, 273, 4, 11],
            10,
        );
        assert_eq!(
            answer[0],
            vec![
                (vec![4, 11], vec![5, 10]),
                (vec![273, 2100], vec![123, 150, 1000, 1100]),
            ]
        );
        assert_eq!(
            answer[1],
            vec![(vec![4, 11, 273, 2100], vec![5, 10, 123, 150, 1000, 1100]),]
        );

        let answer = sequence_matcher_m2m(
            &mut vec![1000, 1100, 150, 123, 5, 10],
            &mut vec![1000, 1200],
            10,
        );
        assert_eq!(answer.len(), 0);

        let answer = sequence_matcher_m2m(&mut vec![-950, 10000], &mut vec![5000, 4000, 50], 10);
        assert_eq!(answer[0], vec![(vec![50, 4000, 5000], vec![-950, 10000]),]);

        let answer = sequence_matcher_m2m(
            &mut vec![99, 68, -74, 72, -38, 22],
            &mut vec![36, -23, -92, 88, 67, 73],
            10,
        );
        assert_eq!(
            answer[2],
            vec![
                (vec![-92, 36, 67, 88], vec![99]),
                (vec![-23, 73], vec![-74, -38, 22, 68, 72]),
            ]
        );

        let answer = sequence_matcher_m2m(&mut vec![1, 2, 3, 4], &mut vec![1, 5], 10);

        assert_eq!(answer.len(), 0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_subset_fast_only_positive() {
        let result = dp::find_subset_fast_only_positive(&vec![1, 2, 3], 3);
        let route1: Vec<u32> = vec![2, 1];
        let route2: Vec<u32> = vec![3];
        let answer: Vec<Vec<u32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset_fast_only_positive(&vec![0, 3, 5, 10], 3);
        let route1: Vec<u32> = vec![3];
        let route2: Vec<u32> = vec![3, 0];
        let answer: Vec<Vec<u32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset_fast_only_positive(&vec![1, 2, 3, 0], 3);
        let route1: Vec<u32> = vec![2, 1];
        let route2: Vec<u32> = vec![3];
        let route3: Vec<u32> = vec![0, 2, 1];
        let route4: Vec<u32> = vec![0, 3];
        let answer: Vec<Vec<u32>> = vec![route1, route2, route3, route4];
        assert_eq!(result, answer);
    }

    #[test]
    fn test_find_test() {
        let result = dp::find_subset(&vec![1, 2, 3], 3);
        let route1: Vec<i32> = vec![3];
        let route2: Vec<i32> = vec![2, 1];
        let answer: Vec<Vec<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset(&vec![1, 2, 3, 4, 5], 10);
        let route1: Vec<i32> = vec![5, 3, 2];
        let route2: Vec<i32> = vec![5, 4, 1];
        let route3: Vec<i32> = vec![4, 3, 2, 1];
        let answer: Vec<Vec<i32>> = vec![route1, route2, route3];
        assert_eq!(result, answer);

        let a = vec![75, 467, 512, -835, 770, -69, 10];
        let result = dp::find_subset(&a, 711);
        let route1: Vec<i32> = vec![10, -69, 770];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![-3, 10, 56, -33, 65, -9, 8, 72, 63, 35];
        let result = dp::find_subset(&a, 7);
        let route1: Vec<i32> = vec![10, -3];
        let route2: Vec<i32> = vec![35, 8, -33, -3];
        let answer: Vec<Vec<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let a = vec![
            73209, 95597, 84735, 40496, 83553, 95595, -628, 201, 27597, 7904, 98445, 6241, 33002,
            -776, -711, 45552, 86746, 84248, 66278, 37475,
        ];
        let result = dp::find_subset(&a, 72782);
        let route1: Vec<i32> = vec![201, -628, 73209];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![-1, 2, 3];
        let result = dp::find_subset(&a, -1);
        let route1: Vec<i32> = vec![-1];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![-10, 5, -2];
        let result = dp::find_subset(&a, -5);
        let route1: Vec<i32> = vec![5, -10];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![-3, -5, -7];
        let result = dp::find_subset(&a, -15);
        let route1: Vec<i32> = vec![-7, -5, -3];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let a = vec![-100, 10, 20];
        let result = dp::find_subset(&a, -70);
        let route1: Vec<i32> = vec![20, 10, -100];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);
    }
}
