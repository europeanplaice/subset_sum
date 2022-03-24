pub mod dp {
    //! This is a module for dynamic programming.

    use itertools::structs::Combinations;

    struct MultiCombination<I: Iterator> {
        combs: Vec<Combinations<I>>,
    }
    
    impl<I> Iterator for MultiCombination<I> 
        where I: Iterator,
        I::Item: Clone  
    {
        type Item = Vec<I::Item>;
    
        fn next(&mut self) -> Option<Self::Item> {
            for comb in &mut self.combs {
                if let Some(elt) = comb.next() {
                    return Some(elt);
                } else {
                    continue
                }
            }
            None
        }
    }
    use std::collections::HashMap;
    use rayon::prelude::*;
    use std::sync::{Arc, Mutex};

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
    /// let result = find_subset(arr, 0, 4);
    /// let route1: Vec<i32> = vec![-3, -2, -1, 6];
    /// let answer: Vec<Vec<i32>> = vec![route1];
    /// assert_eq!(result, answer);
    /// ```
    ///
    /// # Return Value
    /// ```
    ///
    /// use dpss::dp::find_subset;
    /// let result = find_subset(vec![1, 2, 3, -4, 5], 1, 2);
    /// println!("{:?}", result);
    /// ```
    /// output: `[[1], [-3, 4]]`
    pub fn find_subset(arr: Vec<i32>, value: i32, max_length: usize) -> Vec<Vec<i32>> {
        use std::cmp::max;
        use std::cmp::min;
        // https://stackoverflow.com/questions/43078142/subset-sum-with-negative-values-in-c-or-c
        // Find a subset even if an array contains negative values.
        let answer: Arc<Mutex<Vec<Vec<i32>>>> = Arc::new(Mutex::new(vec![]));
        if arr.iter().min().unwrap() >= &0 && value > 0 {
            let result = find_subset_fast_only_positive(&arr.iter().map(|e| *e as u32).collect::<Vec<u32>>(), value as usize, max_length);
            for i in result {
                answer.lock().unwrap().push(i.iter().map(|e| *e as i32).collect::<Vec<i32>>())
            }
            return answer.lock().unwrap().to_vec();
        } else {
            let length = arr.len();
            let offset: i32 =
                (max(arr.iter().min().unwrap().abs() + 1, min(value, 0).abs() + 1)) as u32 as i32;
            // We will transform the array into a new array whose elements are all positive.
            // And check if the transformed sum of the result of the new array is equal to the target value.
            // If we find the sum is the same as the target, we will return the result.
            let c = |i| {
                let result = _find_subset_fast_only_positive(
                    &arr.iter().map(|e| (e + offset) as u32).collect::<Vec<u32>>(),
                    (value + i as i32 * offset) as usize,
                    max_length,
                );
                for res in result {
                    let mut tempsum: i32 = 0;
                    let mut new_res: Vec<i32> = Vec::with_capacity(res.len());
                    for j in &res {
                        tempsum += *j as i32 - offset;
                        new_res.push(*j as i32 - offset);
                    }
                    if tempsum == value {
                        answer.lock().unwrap().push(new_res);
                    }
                }
            };
            if cfg!(feature="wasm"){
                (1..min(length, max_length) + 1).into_iter().for_each(c);
            } else {
                (1..min(length, max_length) + 1).into_par_iter().for_each(c);
            }
            return vector_sorter(answer.lock().unwrap().to_vec());
        };
    }

    fn rec(
        dp: &Vec<u16>,
        arr: &Vec<u32>,
        i: usize,
        j: usize,
        route: &mut Vec<u32>,
        answer: &mut Vec<Vec<u32>>,
        max_length: usize,
        collen: usize,
        last_value: usize
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

        if answer.len() > last_value {
            return;
        }

        if route.len() > max_length {
            return;
        }
        let i_minus_one = i - 1;
        let one_step_up = i_minus_one * collen + j;
        let v = arr[i_minus_one];

        if dp[one_step_up] != 0 {
            rec(dp, arr, i_minus_one, j, route, answer, max_length, collen, last_value);
        }

        let j_v = j as i32 - v as i32;
        if j_v >= 0 && dp[one_step_up - v as usize] != 0 {
            // Choose this element as arr candidate for an answer.
            route.push(v);
            rec(
                dp,
                arr,
                i_minus_one,
                j_v as usize,
                route,
                answer,
                max_length,
                collen, 
                last_value
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
        let answer = _find_subset_fast_only_positive(arr, value, max_length);
        vector_sorter(answer)
    }

    fn _find_subset_fast_only_positive(
        arr: &Vec<u32>,
        value: usize,
        max_length: usize,
    ) -> Vec<Vec<u32>> {
        // dp is a table that stores the information of subset sum.
        // dp[i][j] is the number of ways to make sum j with i element.
        // We follow from the start of this table.
        // let mut dp: Vec<Vec<i32>> = vec![vec![0; value + 1]; arr.len() + 1];
        let collen = value + 1;
        let mut dp: Vec<u16> = vec![0; (value + 1) * (arr.len() + 1)];
        dp[0] = 1;

        let mut current_address= 0;
        arr.iter().for_each(|v_u32| {
            let v_usize = *v_u32 as usize;
            (0..value+1).for_each(|j| {
                let current_value = dp[current_address];
                if current_value == 0 {
                    current_address += 1;
                    return;
                };
                let address_onestep_down = current_address + collen;
                // If we don't choose to select an element to sum,
                // the ways to make a sum are the same as with the previous element.
                // dp[i + 1][*j] += dp[i][*j];
                dp[address_onestep_down] += current_value; 

                // Skip if j + the element is larger than the target value.
                if j + v_usize < collen {
                    // This means we find another way to make sum j with i elements
                    // when we choose this element as an element to sum.
                    // dp[i + 1][j + arr[i] as usize] += dp[i][j];
                    dp[address_onestep_down + v_usize] += current_value; 

                }
                current_address += 1;
            });
        });
        let a_length: usize = arr.len();
        let mut route: Vec<u32> = vec![];
        let mut answer: Vec<Vec<u32>> = vec![];
        rec(
            &dp,
            &arr,
            a_length,
            value,
            &mut route,
            &mut answer,
            max_length,
            collen,
            dp[dp.len() - 1] as usize
        );
        answer
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
    /// * `n_candidates` - An integer.
    /// # Example
    ///
    /// ```rust
    ///
    ///use dpss::dp::sequence_matcher;
    ///let answer = sequence_matcher(&mut vec![1980, 2980, 3500, 4000, 1050], &mut vec![1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10, 10, 100);
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
        n_candidates: usize,
    ) -> Vec<Vec<(Vec<i32>, Vec<i32>)>> {
        let mut group: Vec<(Vec<i32>, Vec<i32>)> = vec![];
        let mut answer: Arc<Mutex<Vec<Vec<(Vec<i32>, Vec<i32>)>>>> = Arc::new(Mutex::new(vec![]));
        if keys.iter().sum::<i32>() != targets.iter().sum() {
            println!("The sum of the keys must be equal to the sum of the targets.");
            return answer.lock().unwrap().to_vec();
        }
        let mut hashmap_fs: Arc<Mutex<HashMap<(Vec<i32>, i32), Vec<Vec<i32>>>>> = Arc::new(Mutex::new(HashMap::new()));
        sequence_matcher_core(
            keys,
            targets,
            &mut group,
            &mut answer,
            max_key_length,
            max_target_length,
            &mut hashmap_fs,
            n_candidates
        );
        let mut answer2 : Vec<Vec<(Vec<i32>, Vec<i32>)>> = answer.lock().unwrap().to_vec();
        for i in 0..answer2.len() {
            answer2[i].sort_by_key(|k| k.0.iter().sum::<i32>());
            answer2[i].sort_by_key(|k| k.0.len());
        }
        answer2.sort();
        answer2.dedup();
        if answer2.len() == 0 {
            println!("Can't find any combination.");
        }
        answer2
    }

    fn sequence_matcher_core(
        keys: &mut Vec<i32>,
        targets: &mut Vec<i32>,
        group: &mut Vec<(Vec<i32>, Vec<i32>)>,
        answer: &mut Arc<Mutex<Vec<Vec<(Vec<i32>, Vec<i32>)>>>>,
        max_key_length: usize,
        max_target_length: usize,
        hashmap_fs: &mut Arc<Mutex<HashMap<(Vec<i32>, i32), Vec<Vec<i32>>>>>,
        n_candidates: usize,
    ) -> (){
        use itertools::Itertools;
        use std::cmp::min;
        if answer.lock().unwrap().len() >= n_candidates {
            return;
        }
        if keys.len() == 0 && targets.len() == 0 {
            group.sort_by_key(|k| k.0.iter().sum::<i32>());
            group.sort_by_key(|k| k.0.len());
            if answer.lock().unwrap().contains(&group) {
                return;
            } else {
                answer.lock().unwrap().push(group.clone());
                return;
            }
        }
        if (keys.len() == 0 && targets.len() > 0) || (keys.len() > 0 && targets.len() == 0) {
            return;
        }
        targets.sort();
        let mut combs = vec![];
        for i in 1..min(max_key_length, keys.len()) + 1{
            combs.push((0..keys.len()).into_iter().combinations(i))
        };
        let mc = MultiCombination{
            combs: combs,
        };
        if cfg!(feature="wasm") {
            mc.for_each(|i| {
                let mut sum_key = 0;
                let mut vec_key = vec![];
                i.iter().for_each(|j| {
                    let k = keys[*j];
                    sum_key += k;
                    vec_key.push(k);
                });
                vec_key.sort();
                if sum_key > targets.iter().sum() {
                    return;
                }
                if sum_key < *targets.iter().min().unwrap() {
                    return;
                }
                if targets.iter().max().unwrap() == &0 {
                    return;
                }
                let mut set_ = match hashmap_fs.try_lock() {
                    Ok(mut v) => {v.entry((targets.clone(), sum_key))
                        .or_insert(find_subset(targets.clone(), sum_key, max_target_length))
                        .clone()},
                    Err(_) => {find_subset(targets.clone(), sum_key, max_target_length)}
                };
                if set_.len() == 0 {
                    return;
                }
                set_.sort();
                set_.dedup();
                set_.iter().for_each(|set| {
                    let mut keys3 = keys.clone();
                    let mut targets3 = targets.clone();
                    let mut group3 = group.clone();
                    group3.push((vec_key.clone(), set.clone()));
                    set.iter().for_each(|j| {
                        vec_remove(&mut targets3, *j);
                    });
                    vec_key.iter().for_each(|i| {
                        vec_remove(&mut keys3, *i);
                    });
                    sequence_matcher_core(
                        &mut keys3,
                        &mut targets3,
                        &mut group3,
                        &mut answer.clone(),
                        max_key_length,
                        max_target_length,
                        &mut hashmap_fs.clone(),
                        n_candidates
                    );
                });
            });
        } else {
            mc.par_bridge().for_each(|i| {
                let mut sum_key = 0;
                let mut vec_key = vec![];
                i.iter().for_each(|j| {
                    let k = keys[*j];
                    sum_key += k;
                    vec_key.push(k);
                });
                vec_key.sort();
                if sum_key > targets.iter().sum() {
                    return;
                }
                if sum_key < *targets.iter().min().unwrap() {
                    return;
                }
                if targets.iter().max().unwrap() == &0 {
                    return;
                }
                let mut set_ = {match hashmap_fs.try_lock() {
                    Ok(mut v) => {v.entry((targets.clone(), sum_key))
                        .or_insert(find_subset(targets.clone(), sum_key, max_target_length))
                        .clone()},
                    Err(_) => {find_subset(targets.clone(), sum_key, max_target_length)}
                }};
                if set_.len() == 0 {
                    return;
                }
                set_.sort();
                set_.dedup();
                set_.par_iter().for_each(|set| {
                    let mut keys3 = keys.clone();
                    let mut targets3 = targets.clone();
                    let mut group3 = group.clone();
                    group3.push((vec_key.clone(), set.clone()));
                    set.iter().for_each(|j| {
                        vec_remove(&mut targets3, *j);
                    });
                    vec_key.iter().for_each(|i| {
                        vec_remove(&mut keys3, *i);
                    });
                    sequence_matcher_core(
                        &mut keys3,
                        &mut targets3,
                        &mut group3,
                        &mut answer.clone(),
                        max_key_length,
                        max_target_length,
                        &mut hashmap_fs.clone(),
                        n_candidates
                    );
                });
            });
        }
        ()
    }

    #[test]
    fn test_sequence_matcher() {
        let answer = sequence_matcher(
            &mut vec![6, 7, 3, 2, -9, -3, 8, 3, 6, -10],
            &mut vec![3, 2, -6, -8, 2, -9, 0, -5, -3, 37],
            7,
            6,
            30
        );
        assert_eq!(answer.len(), 30);

        let answer = sequence_matcher(
            &mut vec![100, 200, 300, 400, 500, 600, -700, 800, 900, 1000],
            &mut vec![300, 700, 500, 600, -700, 2700],
            3,
            2,
            200
        );
        assert_eq!(answer.len(), 195);
        assert_eq!(
            answer[0],
            vec![
                (vec![-700], vec![-700]),
                (vec![100, 200], vec![300]),
                (vec![300, 400], vec![700]),
                (vec![500, 600], vec![500, 600]),
                (vec![800, 900, 1000], vec![2700]),
            ]
        );

        let answer = sequence_matcher(&mut vec![9, 0, 1, 7, 1], &mut vec![7, 2, 8, 0, 1], 3, 2, 100);
        assert_eq!(answer.len(), 24);
        assert_eq!(
            answer[0],
            vec![
                (vec![0], vec![0]),
                (vec![1], vec![1]),
                (vec![7], vec![7]),
                (vec![1, 9], vec![2, 8]),
            ]
        );

        let answer = sequence_matcher(
            &mut vec![1000, 1100, 150, 123, 5, 10],
            &mut vec![2100, 273, 4, 11],
            6,
            4,
            200
        );
        assert_eq!(answer.len(), 5);
        assert_eq!(
            answer[0],
            vec![
                (vec![5, 10], vec![4, 11]),
                (vec![123, 150], vec![273]),
                (vec![1000, 1100], vec![2100]),
            ]
        );
        assert_eq!(
            answer[2],
            vec![
                (vec![5, 10, 123, 150, 1000, 1100], vec![4, 11, 273, 2100]),
            ]
        );

        let answer = sequence_matcher(
            &mut vec![1000, 1100, 150, 123, 5, 10],
            &mut vec![1000, 1200],
            10,
            10,
            2
        );
        assert_eq!(answer.len(), 0);

        let answer = sequence_matcher(&mut vec![-950, 10000], &mut vec![5000, 4000, 50], 10, 10, 2);
        assert_eq!(answer[0], vec![(vec![-950, 10000], vec![50, 4000, 5000]),]);

        let answer = sequence_matcher(&mut vec![1, 2, 3, 4], &mut vec![1, 5], 10, 10, 2);

        assert_eq!(answer.len(), 0);

        let answer = sequence_matcher(
            &mut vec![183, 36, 231, 128, 137],
            &mut vec![8, 9, 15, 15, 33, 36, 39, 45, 46, 60, 68, 73, 80, 92, 96],
            1,
            15,
            20
        );

        assert_eq!(answer.len(), 13);
        assert_eq!(
            answer[0],
            vec![
                (vec![36], vec![36]),
                (vec![128], vec![9, 39, 80]),
                (vec![137], vec![8, 33, 96]),
                (vec![183], vec![45, 46, 92]),
                (vec![231], vec![15, 15, 60, 68, 73]),
            ]
        );
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
        let result = dp::find_subset(vec![1, 2, 3], 3, 2);
        let route1: Vec<i32> = vec![3];
        let route2: Vec<i32> = vec![1, 2];
        let answer: Vec<Vec<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset(vec![1, 2, 3, 4, 5], 10, 4);
        let route1: Vec<i32> = vec![1, 4, 5];
        let route2: Vec<i32> = vec![2, 3, 5];
        let route3: Vec<i32> = vec![1, 2, 3, 4];
        let answer: Vec<Vec<i32>> = vec![route1, route2, route3];
        assert_eq!(result, answer);

        let result = dp::find_subset(vec![1, 2, 3, 4, 5], 10, 3);
        let route2: Vec<i32> = vec![1, 4, 5];
        let route3: Vec<i32> = vec![2, 3, 5];
        let answer: Vec<Vec<i32>> = vec![route2, route3];
        assert_eq!(result, answer);

        let arr = vec![75, 467, 512, -835, 770, -69, 10];
        let result = dp::find_subset(arr, 711, 3);
        let route1: Vec<i32> = vec![-69, 10, 770];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-3, 10, 56, -33, 65, -9, 8, 72, 63, 35];
        let result = dp::find_subset(arr, 7, 4);
        let route1: Vec<i32> = vec![-3, 10];
        let route2: Vec<i32> = vec![-33, -3, 8, 35];
        let answer: Vec<Vec<i32>> = vec![route1, route2];
        assert_eq!(result, answer);

        let arr = vec![
            73209, 95597, 84735, 40496, 83553, 95595, -628, 201, 27597, 7904, 98445, 6241, 33002,
            -776, -711, 45552, 86746, 84248, 66278, 37475,
        ];
        let result = dp::find_subset(arr, 72782, 3);
        let route1: Vec<i32> = vec![-628, 201, 73209];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-1, 2, 3];
        let result = dp::find_subset(arr, -1, 1);
        let route1: Vec<i32> = vec![-1];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-10, 5, -2];
        let result = dp::find_subset(arr, -5, 2);
        let route1: Vec<i32> = vec![-10, 5];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-3, -5, -7];
        let result = dp::find_subset(arr, -15, 3);
        let route1: Vec<i32> = vec![-7, -5, -3];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-100, 10, 20];
        let result = dp::find_subset(arr, -70, 3);
        let route1: Vec<i32> = vec![-100, 10, 20];
        let answer: Vec<Vec<i32>> = vec![route1];
        assert_eq!(result, answer);
    }
}
