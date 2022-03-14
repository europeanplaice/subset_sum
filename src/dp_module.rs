pub mod dp {
    //! This is a module for dynamic programming.
    use std::collections::HashMap;

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
    /// # Arguments
    /// * `arr` - An array.
    /// * `value` - The value to the sum of the subset comes.
    /// * `max_length` - The maximum length of combinations of the answer.
    /// # Example
    /// ```
    ///
    /// use dpss::dp::find_subset;
    /// let arr = vec![-1, -3, -2, 6, 12, 48];
    /// let result = find_subset(&arr, 0, 4);
    /// let route1: Vec<i32> = vec![-3, -2, -1, 6];
    /// let answer: Vec<Vec<i32>> = vec![route1];
    /// assert_eq!(result, answer);
    /// ```
    ///
    /// # Return Value
    /// ```
    ///
    /// use dpss::dp::find_subset;
    /// let result = find_subset(&vec![1, 2, 3, -4, 5], 1, 2);
    /// println!("{:?}", result);
    /// ```
    /// output: `[[1], [-3, 4]]`
    pub fn find_subset(arr: &Vec<i32>, value: i32, max_length: usize) -> Vec<Vec<i32>> {
        use std::cmp::max;
        use std::cmp::min;
        // https://stackoverflow.com/questions/43078142/subset-sum-with-negative-values-in-c-or-c
        // Find a subset even if an array contains negative values.
        let mut b: Vec<u32> = Vec::with_capacity(arr.len());
        let mut answer: Vec<Vec<i32>> = Vec::with_capacity(arr.len());
        if arr.iter().min().unwrap() >= &0 && value > 0 {
            for i in arr {
                b.push(*i as u32);
            }
            let result = find_subset_fast_only_positive(&b, value as usize, max_length);
            for i in result {
                let mut tempvec = Vec::with_capacity(i.len());
                for j in i {
                    tempvec.push(j as i32);
                }
                answer.push(tempvec)
            }
            return answer;
        } else {
            let offset: u32 =
                (max(arr.iter().min().unwrap().abs() + 1, min(value, 0).abs() + 1)) as u32;
            for i in arr {
                b.push((i + offset as i32) as u32);
            }
            // We will transform the array into a new array whose elements are all positive.
            // And check if the transformed sum of the result of the new array is equal to the target value.
            // If we find the sum is the same as the target, we will return the result.
            for i in 1..arr.len() + 1 {
                let result = find_subset_fast_only_positive(
                    &b,
                    (value + i as i32 * offset as i32) as usize,
                    max_length,
                );
                for res in result {
                    let mut tempsum: i32 = 0;
                    let mut new_res: Vec<i32> = Vec::with_capacity(res.len());
                    for el in res {
                        tempsum += el as i32 - offset as i32;
                        new_res.push(el as i32 - offset as i32);
                    }
                    if tempsum == value as i32 {
                        answer.push(new_res);
                    }
                }
            }
            return answer;
        };
    }

    fn rec(
        dp: &Vec<Vec<i32>>,
        arr: &Vec<u32>,
        i: usize,
        j: usize,
        route: &mut Vec<u32>,
        answer: &mut Vec<Vec<u32>>,
        a_min: &u32,
        max_length: usize,
    ) {
        // This code is mostly copied from https://drken1215.hatenablog.com/entry/2019/12/17/190300
        // We follow the dp table backward to find combinations of subsets.
        // We call this function recursively twice and this means the call stack expands like a tree.
        if i == 0 {
            if j == 0 {
                // Only if we reach the root of the dp table, we choose the combination as an answer.
                if route.len() <= max_length {
                    answer.push(route.clone());
                }
            }
            return;
        }

        if route.len() > max_length {
            return;
        }

        if dp[i - 1][j] != 0 {
            rec(dp, arr, i - 1, j, route, answer, a_min, max_length);
        }

        if j as i32 - arr[i - 1] as i32 >= 0 && dp[i - 1][j - arr[i - 1] as usize] != 0 {
            // Choose this element as arr candidate for an answer.
            route.push(arr[i - 1]);
            rec(
                dp,
                arr,
                i - 1,
                j - arr[i - 1] as usize,
                route,
                answer,
                a_min,
                max_length,
            );
            // Remove this element after we reach i == 0 regardless of whether we reach j == 0.
            route.pop();
        }
    }

    fn vector_sorter<T: std::cmp::Ord + std::iter::Sum + std::clone::Clone + Copy>(
        vec: Vec<Vec<T>>,
    ) -> Vec<Vec<T>> {
        if vec.len() == 0 {
            return vec;
        }
        let max_length = vec
            .iter()
            .map(|x| x.len())
            .collect::<Vec<usize>>()
            .iter()
            .max()
            .unwrap()
            .clone();
        let mut newvec: Vec<Vec<T>> = vec![];
        for i in 0..max_length + 1 {
            let mut tempv: Vec<Vec<T>> = vec![];
            for v in vec.iter() {
                if v.len() == i {
                    let mut v_ = v.clone();
                    v_.sort();
                    tempv.push(v_.to_vec());
                }
            }
            for j in (0..i).rev() {
                tempv.sort_by_key(|x| x[j]);
            }
            newvec.append(&mut tempv);
        }
        newvec
    }

    fn filter_j_idx(value: usize, arr: &Vec<u32>) -> (Vec<usize>, u32) {
        // a_min is the minimum number in an except for zero.
        let mut a_min = arr.iter().max().unwrap();
        let mut a_no_zero: Vec<u32> = Vec::with_capacity(arr.len());
        for i in arr {
            if i > &0 {
                if a_min > &i {
                    a_min = &i
                }
                a_no_zero.push(*i);
            }
        }
        let mut j_indexes: Vec<usize> = Vec::with_capacity(value + 1);
        let gcd = gcd_multi(a_no_zero);
        // j of the range of 1 to a_min-1 must be zero.
        // For example, if a_min = 10, there is no way to make sum 5.
        // Also, if j == 8 and target = 10 and a_min=5, we can't reach 10.
        // If all the numbers are even, j should be even.
        for j in 0..value + 1 {
            if (j as u32 >= *a_min && j as u32 <= value as u32 - *a_min && j as u32 % gcd == 0)
                || j as u32 == 0
                || j == value
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
    /// # Arguments
    /// * `arr` - An array.
    /// * `value` - The value to the sum of the subset comes.
    /// * `max_length` - The maximum length of combinations of the answer.
    /// # Example
    /// ```
    ///
    /// use dpss::dp::find_subset_fast_only_positive;
    /// let result = find_subset_fast_only_positive(&vec![1, 2, 3], 3, 2);
    /// let route1: Vec<u32> = vec![3];
    /// let route2: Vec<u32> = vec![1, 2];
    /// let answer: Vec<Vec<u32>> = vec![route1, route2];
    /// assert_eq!(result, answer);
    /// ```
    /// # Return Value
    /// ```
    ///
    /// use dpss::dp::find_subset_fast_only_positive;
    /// let result = find_subset_fast_only_positive(&vec![1, 2, 3, 4, 5], 10, 4);
    /// println!("{:?}", result);
    /// ```
    /// output: `[[1, 4, 5], [2, 3, 5], [1, 2, 3, 4]]`
    pub fn find_subset_fast_only_positive(
        arr: &Vec<u32>,
        value: usize,
        max_length: usize,
    ) -> Vec<Vec<u32>> {
        // dp is a table that stores the information of subset sum.
        // dp[i][j] is the number of ways to make sum j with i element.
        // We follow from the start of this table.
        let mut dp: Vec<Vec<i32>> = vec![vec![0; value + 1]; arr.len() + 1];
        dp[0][0] = 1;

        let (j_indexes, a_min) = filter_j_idx(value, arr);
        for i in 0..arr.len() {
            for j in &j_indexes {
                // If we don't choose to select an element to sum,
                // the ways to make a sum are the same as with the previous element.
                dp[i + 1][*j] += dp[i][*j];

                // Skip if j + the element is larger than the target value.
                if *j as u32 + arr[i] < value as u32 + 1 {
                    // This means we find another way to make sum j with i elements
                    // when we choose this element as an element to sum.
                    dp[i + 1][j + arr[i] as usize] += dp[i][*j];
                }
            }
        }
        let a_length: usize = arr.len();
        let mut route: Vec<u32> = Vec::with_capacity(a_length);
        let mut answer: Vec<Vec<u32>> = Vec::with_capacity(a_length);

        rec(
            &dp,
            &arr,
            a_length,
            value,
            &mut route,
            &mut answer,
            &a_min,
            max_length,
        );
        // for i in answer.iter_mut(){
        //     i.sort();
        // };
        // answer.sort_by_key(|k| k[0]);
        // answer.sort_by_key(|k| k.len());
        vector_sorter(answer)
        // answer
    }

    fn vec_remove(arr: &mut Vec<i32>, v: i32) {
        let index = arr.iter().position(|x| *x == v).unwrap();
        arr.remove(index);
    }

    /// Finds the integers from two vectors that sum to the same value.
    /// This method assumes that the two vectors have Many-to-Many relationships.
    /// Each integer of the `keys` vector corresponds to the multiple integers of the `targets` vector.
    /// With this method, we can find combinations of the integers.
    /// # Arguments
    /// * `keys` - An array.
    /// * `targets` - An array.
    /// * `max_key_length` - An integer.
    /// * `max_target_length` - An integer.
    /// # Example
    ///
    /// ```rust
    ///
    ///use dpss::dp::sequence_matcher;
    ///let answer = sequence_matcher(&mut vec![1980, 2980, 3500, 4000, 1050], &mut vec![1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10, 10);
    ///assert_eq!(answer[0], vec![
    ///    (vec![1050],
    ///     vec![1050]),
    ///
    ///     (vec![1980],
    ///     vec![30, 1950]),
    ///
    ///     (vec![2980],
    ///     vec![80, 2900]),
    ///
    ///     (vec![3500],
    ///     vec![200, 3300]),
    ///
    ///     (vec![4000],
    ///     vec![20, 3980]),
    ///
    ///    ]);
    ///assert_eq!(answer[1], vec![
    ///    (vec![1050],
    ///     vec![1050]),
    ///
    ///     (vec![1980],
    ///     vec![30, 1950]),
    ///
    ///     (vec![2980],
    ///     vec![80, 2900]),
    ///
    ///     (vec![3500, 4000],
    ///     vec![20, 200, 3300, 3980]),
    ///
    ///    ]);
    /// ```
    pub fn sequence_matcher(
        keys: &mut Vec<i32>,
        targets: &mut Vec<i32>,
        max_key_length: usize,
        max_target_length: usize,
    ) -> Vec<Vec<(Vec<i32>, Vec<i32>)>> {
        let mut group: Vec<(Vec<i32>, Vec<i32>)> = Vec::with_capacity(targets.len());
        let mut answer: Vec<Vec<(Vec<i32>, Vec<i32>)>> = vec![];
        if keys.iter().sum::<i32>() != targets.iter().sum() {
            println!("The sum of the keys must be equal to the sum of the targets.");
            return answer;
        }
        let mut hashmap_fs: HashMap<(Vec<i32>, i32), Vec<Vec<i32>>> = HashMap::new();
        let mut key_target_group: HashMap<(Vec<i32>, Vec<i32>), Vec<(Vec<i32>, Vec<i32>)>> =
            HashMap::new();
        sequsequence_matcher_core(
            keys,
            targets,
            &mut group,
            &mut answer,
            1,
            max_key_length,
            max_target_length,
            &mut hashmap_fs,
            &mut key_target_group,
        );
        for i in 0..answer.len() {
            answer[i].sort_by_key(|k| k.0.iter().sum::<i32>());
            answer[i].sort_by_key(|k| k.0.len());
        }
        answer.sort();
        answer.dedup();
        if answer.len() == 0 {
            println!("Can't find any combination.");
        }
        answer
    }

    fn sequsequence_matcher_core(
        keys: &mut Vec<i32>,
        targets: &mut Vec<i32>,
        group: &mut Vec<(Vec<i32>, Vec<i32>)>,
        answer: &mut Vec<Vec<(Vec<i32>, Vec<i32>)>>,
        n_key: usize,
        max_key_length: usize,
        max_target_length: usize,
        hashmap_fs: &mut HashMap<(Vec<i32>, i32), Vec<Vec<i32>>>,
        key_target_group: &mut HashMap<(Vec<i32>, Vec<i32>), Vec<(Vec<i32>, Vec<i32>)>>,
    ) {
        use itertools::Itertools;

        if keys.iter().sum::<i32>() != targets.iter().sum() {
            return;
        }

        if keys.len() == 0 && targets.len() == 0 {
            answer.push(group.clone());
            return;
        }
        if (keys.len() == 0 && targets.len() > 0) || (keys.len() > 0 && targets.len() == 0) {
            return;
        }

        if n_key > max_key_length || n_key > keys.len() {
            return;
        }

        let key_candidates: Vec<Vec<usize>> = (0..keys.len())
            .powerset()
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|x| x.len() <= max_key_length)
            .collect();
        let goldkey = keys.clone();
        let goldtargets = targets.clone();
        let goldgroup = group.clone();
        key_candidates.iter().for_each(|i| {
            *keys = goldkey.clone();
            *targets = goldtargets.clone();
            *group = goldgroup.clone();
            let mut sum_key = 0;
            let mut vec_key = vec![];
            for j in i.iter() {
                if j >= &keys.len() {
                    return;
                }
                sum_key += keys[*j];
                vec_key.push(keys[*j].clone());
            }
            if targets.iter().max().unwrap() == &0 {
                return;
            }
            targets.sort();
            let set_ = hashmap_fs
                .entry((targets.clone(), sum_key.clone()))
                .or_insert(find_subset(&targets, sum_key, max_target_length))
                .clone();
            if set_.len() == 0 {
                return;
            }
            for set in set_ {
                let mut _set = Vec::from(set.clone());
                _set.sort();
                let mut _vec_key = vec_key.clone();
                _vec_key.sort();
                group.push((_vec_key, _set));
                for el in set.clone() {
                    if targets.contains(&el) == false {
                        return;
                    }
                    vec_remove(targets, el);
                }
                for i in vec_key.clone() {
                    if keys.contains(&i) == false {
                        return;
                    }
                    vec_remove(keys, i);
                }
                // if keys.len() > 0 && targets.len() > 0 {
                //     if key_target_group.contains_key(&(keys.clone(), targets.clone())) {
                //         group.push((keys.clone(), targets.clone()));
                //         for d in key_target_group
                //             .get(&(keys.clone(), targets.clone()))
                //             .unwrap()
                //         {
                //             if group.contains(&d) == false {
                //                 group.push(d.clone());
                //             }
                //             for i in &d.0 {
                //                 if keys.contains(&i) == false {
                //                     continue;
                //                 }
                //                 vec_remove(keys, *i);
                //             }
                //             for el in &d.1 {
                //                 if targets.contains(&el) == false {
                //                     continue;
                //                 }
                //                 vec_remove(targets, *el);
                //             }
                //         }
                //         answer.push(group.clone());
                //         println!("ok");
                //         return;
                //     } else {
                //         key_target_group.insert((keys.clone(), targets.clone()), group.clone());
                //     }
                // }
                sequsequence_matcher_core(
                    keys,
                    targets,
                    group,
                    answer,
                    n_key,
                    max_key_length,
                    max_target_length,
                    hashmap_fs,
                    key_target_group,
                );
                group.pop();
                for el in set.clone() {
                    targets.push(el);
                }
                for i in vec_key.clone() {
                    keys.push(i);
                }
            }
            return;
        });
    }

    #[test]
    fn test_sequence_matcher() {
        let answer = sequence_matcher(
            &mut vec![6, 7, 3, 2, -9, -3, 8, 3, 6, -10],
            &mut vec![3, 2, -6, -8, 2, -9, 0, -5, -3, 37],
            10,
            10,
        );
        assert_eq!(
            answer[0],
            vec![
                (vec![], vec![-5, 2, 3]),
                (vec![], vec![0]),
                (
                    vec![-10, -9, -3, 2, 3, 3, 6, 6, 7, 8],
                    vec![-9, -8, -6, -3, 2, 37]
                ),
            ]
        );

        let answer = sequence_matcher(
            &mut vec![6, 7, 3, 2, -9],
            &mut vec![-3, 8, 3, 6, -5],
            10,
            10,
        );
        assert_eq!(
            answer[0],
            vec![
                (vec![], vec![-5, -3, 8]),
                (vec![-9, 2, 3, 6, 7], vec![3, 6]),
            ]
        );

        let answer = sequence_matcher(&mut vec![9, 0, 1, 7, 1], &mut vec![7, 2, 8, 0, 1], 10, 10);
        assert_eq!(
            answer[0],
            vec![
                (vec![], vec![0]),
                (vec![0, 1], vec![1]),
                (vec![1, 7, 9], vec![2, 7, 8]),
            ]
        );

        let answer = sequence_matcher(
            &mut vec![1, 2, 3, 4, 5],
            &mut vec![11, -8, 14, -7, 5],
            10,
            10,
        );
        assert_eq!(
            answer[0],
            vec![(vec![1], vec![-8, -7, 5, 11]), (vec![2, 3, 4, 5], vec![14]),]
        );

        let answer = sequence_matcher(
            &mut vec![1000, 1100, 150, 123, 5, 10],
            &mut vec![2100, 273, 4, 11],
            10,
            10,
        );
        assert_eq!(
            answer[0],
            vec![
                (vec![5, 10], vec![4, 11]),
                (vec![123, 150], vec![273]),
                (vec![1000, 1100], vec![2100]),
            ]
        );
        assert_eq!(
            answer[1],
            vec![
                (vec![5, 10], vec![4, 11]),
                (vec![123, 150, 1000, 1100], vec![273, 2100]),
            ]
        );

        let answer = sequence_matcher(
            &mut vec![1000, 1100, 150, 123, 5, 10],
            &mut vec![1000, 1200],
            10,
            10,
        );
        assert_eq!(answer.len(), 0);

        let answer = sequence_matcher(&mut vec![-950, 10000], &mut vec![5000, 4000, 50], 10, 10);
        assert_eq!(answer[0], vec![(vec![-950, 10000], vec![50, 4000, 5000]),]);

        let answer = sequence_matcher(
            &mut vec![99, 68, -74, 72, -38, 22],
            &mut vec![36, -23, -92, 88, 67, 73],
            10,
            10,
        );
        assert_eq!(
            answer[0],
            vec![(
                vec![-74, -38, 22, 68, 72, 99],
                vec![-92, -23, 36, 67, 73, 88]
            ),]
        );

        let answer = sequence_matcher(&mut vec![1, 2, 3, 4], &mut vec![1, 5], 10, 10);

        assert_eq!(answer.len(), 0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_subset_fast_only_positive() {
        let result = dp::find_subset_fast_only_positive(&vec![1, 2, 3], 3, 2);
        let route1: Vec<u32> = vec![3];
        let route2: Vec<u32> = vec![1, 2];
        let answer: Vec<Vec<u32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset_fast_only_positive(&vec![0, 3, 5, 10], 3, 2);
        let route1: Vec<u32> = vec![3];
        let route2: Vec<u32> = vec![0, 3];
        let answer: Vec<Vec<u32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset_fast_only_positive(&vec![1, 2, 3, 0], 3, 3);
        let route1: Vec<u32> = vec![3];
        let route2: Vec<u32> = vec![0, 3];
        let route3: Vec<u32> = vec![1, 2];
        let route4: Vec<u32> = vec![0, 1, 2];
        let answer: Vec<Vec<u32>> = vec![route1, route2, route3, route4];
        assert_eq!(result, answer);
    }

    #[test]
    fn test_find_subset() {
        let result = dp::find_subset(&vec![1, 2, 3], 3, 2);
        let route1: Vec<i32> = vec![3];
        let route2: Vec<i32> = vec![1, 2];
        let answer: Vec<Vec<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset(&vec![1, 2, 3, 4, 5], 10, 4);
        let route1: Vec<i32> = vec![1, 4, 5];
        let route2: Vec<i32> = vec![2, 3, 5];
        let route3: Vec<i32> = vec![1, 2, 3, 4];
        let answer: Vec<Vec<i32>> = vec![route1, route2, route3];
        assert_eq!(result, answer);

        let result = dp::find_subset(&vec![1, 2, 3, 4, 5], 10, 3);
        let route2: Vec<i32> = vec![1, 4, 5];
        let route3: Vec<i32> = vec![2, 3, 5];
        let answer: Vec<Vec<i32>> = vec![route2, route3];
        assert_eq!(result, answer);

        let arr = vec![75, 467, 512, -835, 770, -69, 10];
        let result = dp::find_subset(&arr, 711, 3);
        let route1: Vec<i32> = vec![-69, 10, 770];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-3, 10, 56, -33, 65, -9, 8, 72, 63, 35];
        let result = dp::find_subset(&arr, 7, 4);
        let route1: Vec<i32> = vec![-3, 10];
        let route2: Vec<i32> = vec![-33, -3, 8, 35];
        let answer: Vec<Vec<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let arr = vec![
            73209, 95597, 84735, 40496, 83553, 95595, -628, 201, 27597, 7904, 98445, 6241, 33002,
            -776, -711, 45552, 86746, 84248, 66278, 37475,
        ];
        let result = dp::find_subset(&arr, 72782, 3);
        let route1: Vec<i32> = vec![-628, 201, 73209];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-1, 2, 3];
        let result = dp::find_subset(&arr, -1, 1);
        let route1: Vec<i32> = vec![-1];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-10, 5, -2];
        let result = dp::find_subset(&arr, -5, 2);
        let route1: Vec<i32> = vec![-10, 5];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-3, -5, -7];
        let result = dp::find_subset(&arr, -15, 3);
        let route1: Vec<i32> = vec![-7, -5, -3];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-100, 10, 20];
        let result = dp::find_subset(&arr, -70, 3);
        let route1: Vec<i32> = vec![-100, 10, 20];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);
    }
}
