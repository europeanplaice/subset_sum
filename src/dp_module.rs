pub mod dp {
    //! This is a module for dynamic programming.

    use itertools::structs::Combinations;
    use std::collections::{HashMap};
    use std::sync::RwLock;
    use field_accessor::FieldAccessor;


    struct MultiCombination<I: Iterator> {
        combs: Vec<Combinations<I>>,
    }

    impl<I> Iterator for MultiCombination<I>
    where
        I: Iterator,
        I::Item: Clone,
    {
        type Item = Vec<I::Item>;
        fn next(&mut self) -> Option<Self::Item> {
            for comb in &mut self.combs {
                if let Some(elt) = comb.next() {
                    return Some(elt);
                } else {
                    continue;
                }
            }
            None
        }
    }
    use rayon::prelude::*;
    use std::sync::Arc;

    #[derive(Clone, Debug, FieldAccessor)]
    pub struct AnswerElement {
        pub answer_arr: Vec<(Vec<i64>, Vec<i64>)>,
        pub keys_remainder: Vec<i64>,
        pub targets_remainder: Vec<i64>,
    }

    impl Eq for AnswerElement {}

    impl PartialEq for AnswerElement {
        fn eq(&self, other: &Self) -> bool {
            self.answer_arr == other.answer_arr
        }
    }

    use std::cmp::Ordering;

    impl Ord for AnswerElement {
        fn cmp(&self, other: &Self) -> Ordering {
            self.answer_arr.cmp(&other.answer_arr)
        }
    }

    impl PartialOrd for AnswerElement {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Clone, Debug)]
    struct DpTable {
        dp: Arc<DashSet<usize>>,
        length: usize,
        max_value: usize,
    }

    pub fn sequence_matcher_formatter(result: Vec<AnswerElement>) -> String {
        let mut s: Vec<String> = vec![];
        for (i, r) in result.iter().enumerate() {
            let mut t: Vec<String> = vec![];
            for elem in r.answer_arr.clone() {
                let key_str: String = elem
                    .0
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>()
                    .join(" + ");
                let target_str: String = elem
                    .1
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>()
                    .join(" + ");
                t.push(format!(
                    "(Sum({}) -> keys:[{}] == targets:[{}])",
                    elem.0.iter().sum::<i64>(),
                    key_str,
                    target_str
                ));
            }
            s.push(format!(
                "pattern{number:^width$}=> [{v}],\n               keys remainder    : {k}\n               targets remainder : {t}\n",
                number = i + 1,
                width = 4,
                v = t.join("\n               "),
                k = r.keys_remainder.iter().map(|k| k.to_string()).collect::<Vec<String>>().join(", "),
                t = r.targets_remainder.iter().map(|k| k.to_string()).collect::<Vec<String>>().join(", "),
            ))
        }
        s.join("\n")
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
    /// let result = find_subset(arr, 0, 4);
    /// let route1: Vec<i64> = vec![-3, -2, -1, 6];
    /// let answer: Vec<Vec<i64>> = vec![route1];
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
    pub fn find_subset(arr: Vec<i64>, value: i64, max_length: usize) -> Vec<Vec<i64>> {
        _find_subset(&arr, value, max_length, None, None, None)
    }

    fn _find_subset(
        arr: &Vec<i64>,
        value: i64,
        max_length: usize,
        dptable: Option<&DpTable>,
        arr_pos: Option<&Vec<u64>>,
        offset: Option<i64>,
    ) -> Vec<Vec<i64>> {
        use std::cmp::max;
        use std::cmp::min;
        // https://stackoverflow.com/questions/43078142/subset-sum-with-negative-values-in-c-or-c
        // Find a subset even if an array contains negative values.
        let offset: i64 = match offset {
            Some(x) => x,
            None => {
                (max(arr.iter().min().unwrap().abs() + 1, min(value, 0).abs() + 1)) as i64
            }
        };
        let answer: Arc<RwLock<Vec<Vec<i64>>>> = Arc::new(RwLock::new(vec![]));
        if offset == 0 && arr.iter().min().unwrap() >= &0 && value >= 0 {
            let arr_pos = arr.iter().map(|e| *e as u64).collect::<Vec<u64>>();
            let _dp: DpTable;
            let dptable = match dptable {
                Some(x) => x,
                None => {
                    _dp = _make_dp_table(&arr_pos, value as usize);
                    &_dp
                }
            };
            let result =
                _find_subsetsfast_only_positive(&arr_pos, value as usize, max_length, dptable);
            result.iter().for_each(|i| {
                answer
                    .write()
                    .unwrap()
                    .push(i.iter().map(|e| *e as i64).collect::<Vec<i64>>())
            });
            return vector_sorter(answer.read().unwrap().to_vec());
        } else {
            let length = arr.len();

            // We will transform the array into a new array whose elements are all positive.
            // And check if the transformed sum of the result of the new array is equal to the target value.
            // If we find the sum is the same as the target, we will return the result.
            let max_value = value + min(length, max_length) as i64 * offset;
            let _arr_pos: &Vec<u64>;
            let temp;
            _arr_pos = match arr_pos {
                None => {
                    temp = arr
                        .iter()
                        .map(|e| (e + offset) as u64)
                        .collect::<Vec<u64>>();
                    &temp
                }
                Some(x) => x,
            };
            let temp2;
            let dptable = match dptable {
                Some(x) => x,
                None => {
                    temp2 = _make_dp_table(&_arr_pos, max_value as usize);
                    &temp2
                }
            };
            let c = |i| {
                let result = _find_subsetsfast_only_positive(
                    &_arr_pos,
                    (value + (i as i64 * offset)) as usize,
                    max_length,
                    dptable,
                );
                result.iter().for_each(|res| {
                    let mut tempsum: i64 = 0;
                    let mut new_res: Vec<i64> = Vec::with_capacity(res.len());
                    for j in res {
                        let v = *j as i64 - offset;
                        tempsum += v;
                        new_res.push(v);
                    }
                    if tempsum == value {
                        answer.write().unwrap().push(new_res);
                    }
                })
            };
            if cfg!(feature = "wasm") {
                (1..min(length, max_length) + 1).into_iter().for_each(c);
            } else {
                (1..min(length, max_length) + 1).into_par_iter().for_each(c);
            }
            return vector_sorter(answer.read().unwrap().to_vec());
        };
    }

    fn rec(
        dp: &Arc<DashSet<usize>>,
        arr: &Vec<u64>,
        i: usize,
        j: usize,
        route: &mut Vec<u64>,
        answer: &mut Vec<Vec<u64>>,
        max_length: usize,
        collen: usize,
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
        let i_minus_one = i - 1;
        let one_step_up = i_minus_one * collen + j;
        let v = { *arr.get(i_minus_one).unwrap() };

        // println!("one_step_up dp {:?}", dp);
        if dp.contains(&one_step_up) {
            rec(dp, arr, i_minus_one, j, route, answer, max_length, collen);
        }

        let j_v: usize = match j.checked_sub(v as usize) {
            Some(x) => x,
            None => return,
        };
        if dp.contains(&(one_step_up - v as usize)) {
            route.push(v);
            rec(dp, arr, i_minus_one, j_v, route, answer, max_length, collen);
            route.pop();
        }
    }

    #[test]

    fn test_vector_sorter() {
        for _i in 0..1 {
            let v = vec![vec![1, 3, 4], vec![4, 5]];
            let r = vector_sorter(v);
            assert_eq!(r, vec![vec![4, 5], vec![1, 3, 4]]);

            let v = vec![vec![4, 2, 1], vec![4, 2]];
            let r = vector_sorter(v);
            assert_eq!(r, vec![vec![2, 4], vec![1, 2, 4]]);

            let v = vec![vec![3, 0], vec![3], vec![2, 1], vec![1, 0, 2]];
            let r = vector_sorter(v);
            assert_eq!(r, vec![vec![3], vec![1, 2], vec![0, 3], vec![0, 1, 2]]);
        }
    }

    fn vector_sorter<T: std::cmp::Ord + std::iter::Sum + std::clone::Clone + Copy>(
        mut vec: Vec<Vec<T>>,
    ) -> Vec<Vec<T>> {
        if vec.len() == 0 {
            return vec;
        }
        vec.sort_unstable();
        vec.sort_unstable_by_key(|x| x.len());
        vec.iter_mut().for_each(|x| x.sort_unstable());
        vec
    }

    use dashmap::DashSet;

    fn add_indexed_recursive(arr: &Vec<u64>, dp: &Arc<DashSet<usize>>, addr: usize, collen: usize, depth: usize, length: usize) {
        if depth == arr.len() || addr > length {
            return;
        }
    
        let stepsize = arr[depth] as usize;
        let new_addr = addr + collen;
        let new_addr_step = new_addr + stepsize;
    
        let mut new_tasks = vec![];
    
        if dp.insert(new_addr) {
            new_tasks.push((new_addr, depth + 1));
        }
    
        if dp.insert(new_addr_step) {
            new_tasks.push((new_addr_step, depth + 1));
        }
    
        if depth < 4 {
            // Parallel execution
            new_tasks.into_par_iter().for_each(|(next_addr, next_depth)| {
                add_indexed_recursive(arr, &dp, next_addr, collen, next_depth, length);
            });
        } else {
            // Sequential execution
            for (next_addr, next_depth) in new_tasks {
                add_indexed_recursive(arr, &dp, next_addr, collen, next_depth, length);
            }
        }
    }
    
    fn _make_dp_table(arr: &Vec<u64>, value: usize) -> DpTable {
        // This function is used to create a dp table.
        // The dp table is a table that stores the information of subset sum.
        // However, dp table is sparse, so we use a hash set to store the information to reduce memory usase.
        // dp only stores the indexes of the dp table that contains true value.
        // To reduce computation time, we don't use nested loop, but recursive function.

        let collen = value + 1;
        let dp = Arc::new(DashSet::new());
        dp.insert(0);
        
        // let mut current_address = 0;
        // println!("start add_indexed_recursive");
        let length = (value + 1) * (arr.len() + 1);
        add_indexed_recursive(arr, &dp, 0, collen, 0, length);
        // println!("finish add_indexed_recursive");
        DpTable {
            dp,
            length,
            max_value: value,
        }
        
    }

    // fn _make_dp_table(arr: &Vec<u64>, value: usize) -> DpTable {
    //     let collen = value + 1;
    //     let mut dp  = BTreeSet::new();
    //     // let mut dp = BitVec::repeat(false, (value + 1) * (arr.len() + 1));
    //     // dp.set(0, true);
    //     // dp.reserve(value + 1);
    //     dp.insert(0);
    //     let mut current_address = 0;
    //     let mut address_onestep_down = 0;
    //     let mut arridx = 0;
    //     arr.iter().for_each(|v_u32| {
    //         let threshold = collen * arridx + (arr.iter().take(arridx + 1).sum::<u64>() + 1) as usize;
    //         let v_usize = *v_u32 as usize;
    //         for _j in 0..value + 1 {
    //             let index = dp.iter().position(|&x| x == current_address).unwrap();

    //             if dp.contains(&current_address) {
    //                 address_onestep_down = current_address + collen;
    //                 dp.insert(address_onestep_down);
    //                 dp.insert(address_onestep_down + v_usize);
    //                 current_address += 1;
    //             } else {
    //                 if current_address == threshold {
    //                     // After the threshold, all values are false.
    //                     current_address = collen * (arridx + 1);
    //                     break
    //                 } else {
    //                     current_address += 1;
    //                     continue;
    //                 }
    //             }
    //         }
    //         arridx += 1;
    //     });
    //     DpTable {
    //         dp,
    //         length: (value + 1) * (arr.len() + 1) as usize,
    //         max_value: value,
    //     }
    // }
    fn _find_subsetsfast_only_positive(
        arr: &Vec<u64>,
        value: usize,
        max_length: usize,
        dptable: &DpTable,
    ) -> Vec<Vec<u64>> {
        // dp is a table that stores the information of subset sum.
        // dp[i][j] is the number of ways to make sum j with i element.
        // We follow from the start of this table.
        // let mut dp: Vec<Vec<i64>> = vec![vec![0; value + 1]; arr.len() + 1];
        let answer_exist: bool = dptable
            .dp
            .contains(&(dptable.length - 1 - (dptable.max_value - value)));
        if answer_exist == false {
            return vec![];
        }
        let collen = dptable.max_value + 1;
        let a_length: usize = arr.len();
        let mut route: Vec<u64> = vec![];
        let mut answer: Vec<Vec<u64>> = vec![];
        rec(
            &dptable.dp,
            &arr,
            a_length,
            value,
            &mut route,
            &mut answer,
            max_length,
            collen,
        );
        answer
    }

    fn vec_remove(arr: &mut Vec<i64>, v: i64) {
        let index = arr.binary_search(&v).unwrap();
        arr.remove(index);
    }

    /// Finds the integers from two vectors that sum to the same value.
    /// This method assumes that the two vectors have Many-to-Many relationships.
    /// Each integer of the `keys` vector corresponds to the multiple integers of the `targets` vector.
    /// With this method, we can find combinations of the integers.
    ///
    /// To avoid combinatorial explosion, some parameters need to be set.
    /// `max_key_length` is used to restrict the number of values in keys chosen.
    /// If `max_key_length` is 3, an answer's length is at most 3, such as `[1980 + 2980 + 3500], [1050]`
    /// `max_target_length` is the same as `max_key_length` for targets.
    /// `n_candidates` specifies the maximum number of pattern.
    /// If `use_all_keys` is true, an answer must contain all the elements of the keys.
    /// If `use_all_targets` is true, an answer must contain all the elements of the targets.
    /// When both `use_all_keys` and `use_all_targets` are true, the sum of the keys and the targets must be the same.
    ///
    /// # Arguments
    /// * `keys` - An array.
    /// * `targets` - An array.
    /// * `max_key_length` - An integer.
    /// * `max_target_length` - An integer.
    /// * `n_candidates` - An integer.
    /// * `use_all_keys` - Boolean.
    /// * `use_all_targets` - Boolean.
    /// # Example
    ///
    /// ```rust
    ///
    ///use dpss::dp::sequence_matcher;
    ///let answer = sequence_matcher(&mut vec![1980, 2980, 3500, 4000, 1050], &mut vec![1950, 2900, 30, 80, 3300, 200, 3980, 1050, 20], 10, 10, 100, true, true).unwrap();
    ///assert_eq!(answer[0].answer_arr, vec![
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
    ///assert_eq!(answer[1].answer_arr, vec![
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
        keys: &mut Vec<i64>,
        targets: &mut Vec<i64>,
        max_key_length: usize,
        max_target_length: usize,
        n_candidates: usize,
        use_all_keys: bool,
        use_all_targets: bool,
    ) -> Result<Vec<AnswerElement>, String> {
        let mut group: Vec<(Vec<i64>, Vec<i64>)> = vec![];
        let mut answer: Arc<RwLock<Vec<AnswerElement>>> = Arc::new(RwLock::new(vec![]));
        if use_all_keys && use_all_targets {
            let ks = keys.iter().sum::<i64>();
            let ts = targets.iter().sum::<i64>();
            if ks != ts {
                return Err(format!("The sums of two arrays must be the same values. key's sum is {}. target's sum is {}. The difference is {}.", ks, ts, ks - ts));
            }
        }
        let mut hashmap_fs: Arc<RwLock<HashMap<(Vec<i64>, i64), Vec<Vec<i64>>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        keys.sort_unstable();
        targets.sort_unstable();
        let swap: bool;
        let (keys, targets, max_key_length, max_target_length, use_all_keys, use_all_targets) =
            if keys.len() > targets.len() {
                swap = true;
                (
                    targets,
                    keys,
                    max_target_length,
                    max_key_length,
                    use_all_targets,
                    use_all_keys,
                )
            } else {
                swap = false;
                (
                    keys,
                    targets,
                    max_key_length,
                    max_target_length,
                    use_all_keys,
                    use_all_targets,
                )
            };
        use std::cmp::min;
        (0..min(keys.len(), targets.len())).for_each(|i| {
            sequence_matcher_core(
                keys,
                targets,
                &mut group,
                &mut answer,
                max_key_length,
                max_target_length,
                &mut hashmap_fs,
                n_candidates,
                use_all_keys,
                use_all_targets,
                i,
                vec![],
            )
        });
        let mut answer_vec: Vec<AnswerElement> = answer.read().unwrap().to_vec();
        if swap {
            answer_vec.iter_mut().for_each(|x| {
                x.answer_arr.iter_mut().for_each(|y| {
                    std::mem::swap(&mut y.0, &mut y.1);
                });
                match x.swap(&"keys_remainder".to_string(), &"targets_remainder".to_string()) {
                    Ok(_) => {},
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            });
        }
        for i in 0..answer_vec.len() {
            answer_vec[i]
                .answer_arr
                .sort_unstable_by_key(|k| k.0.iter().sum::<i64>());
            answer_vec[i].answer_arr.sort_unstable_by_key(|k| k.0.len());
        }
        answer_vec.sort_unstable();
        answer_vec.dedup();
        if answer_vec.len() == 0 {
            println!("Can't find any combination.");
        }
        Ok(answer_vec[..min(n_candidates, answer_vec.len())].to_vec())
    }

    fn sequence_matcher_core(
        keys: &mut Vec<i64>,
        targets: &mut Vec<i64>,
        group: &mut Vec<(Vec<i64>, Vec<i64>)>,
        answer: &mut Arc<RwLock<Vec<AnswerElement>>>,
        max_key_length: usize,
        max_target_length: usize,
        hashmap_fs: &mut Arc<RwLock<HashMap<(Vec<i64>, i64), Vec<Vec<i64>>>>>,
        n_candidates: usize,
        use_all_keys: bool,
        use_all_targets: bool,
        max_depth: usize,
        last_key: Vec<i64>,
    ) -> () {
        use itertools::Itertools;
        use std::cmp::max;
        use std::cmp::min;

        if answer.read().unwrap().len() >= n_candidates {
            return;
        }

        let add: bool = match (use_all_keys, use_all_targets) {
            (true, true) => {
                let add = match (keys.len() == 0, targets.len() == 0) {
                    (true, true) => true,
                    _ => false,
                };
                add
            }
            (true, false) => {
                let add = match (keys.len() == 0, targets.len() == 0) {
                    (true, true) => true,
                    (true, false) => true,
                    _ => false,
                };
                add
            }
            (false, true) => {
                let add = match (keys.len() == 0, targets.len() == 0) {
                    (true, true) => true,
                    (false, true) => true,
                    _ => false,
                };
                add
            }
            (false, false) => {
                let add = match (keys.len() == 0, targets.len() == 0) {
                    (true, true) => true,
                    (false, true) => true,
                    (true, false) => true,
                    _ => false,
                };
                add
            }
        };

        if add {
            group.sort_unstable_by_key(|k| k.0.iter().sum::<i64>());
            group.sort_unstable_by_key(|k| k.0.len());
            let elem = AnswerElement {
                answer_arr: group.clone(),
                keys_remainder: keys.clone(),
                targets_remainder: targets.clone(),
            };
            if answer.read().unwrap().contains(&elem) {
                return;
            } else {
                answer.write().unwrap().push(elem.clone());
                return;
            }
        }
        if keys.len() == 0 || targets.len() == 0 {
            return;
        }
        if group.len() > max_depth {
            return;
        }
        targets.sort_unstable();
        keys.sort_unstable();
        keys.reverse();
        let dp: DpTable;
        let mut offset: i64 = 0;
        let arr_pos: Vec<u64>;
        let dp2: Option<&DpTable> =
            if targets.iter().min().unwrap() >= &0 && keys.iter().min().unwrap() >= &0 {
                arr_pos = targets.iter().map(|e| *e as u64).collect::<Vec<u64>>();
                let max_value = keys[..min(max_key_length, keys.len())].iter().sum::<i64>();
                dp = _make_dp_table(&arr_pos, max_value as usize);
                Some(&dp)
            } else {
                offset = max(
                    targets.iter().min().unwrap().abs() + 1,
                    keys.iter().fold(0, |sum, x| min(sum, x + sum)).abs() + 1,
                );
                arr_pos = targets
                    .iter()
                    .map(|e| (e + offset) as u64)
                    .collect::<Vec<u64>>();
                let _max_value = keys[..min(max_key_length, keys.len())]
                    .iter()
                    .map(|x| max(0, *x))
                    .sum::<i64>();
                let max_value = _max_value + min(targets.len(), max_target_length) as i64 * offset;
                dp = _make_dp_table(&arr_pos, max_value as usize);
                Some(&dp)
            };
        keys.reverse();
        let mut combs = vec![];
        for i in 1..min(max_key_length, keys.len()) + 1 {
            combs.push(keys.clone().into_iter().enumerate().combinations(i))
        }
        let mc = MultiCombination { combs: combs };
        if cfg!(feature = "wasm") {
            mc.for_each(|i| {
                let sum_key: i64 = i.iter().map(|j| j.1).sum();
                if sum_key < last_key.iter().sum() && i.len() == last_key.len() {
                    return;
                }
                let mut sets = {
                    match hashmap_fs.try_write() {
                        Ok(mut v) => v
                            .entry((targets.clone(), sum_key))
                            .or_insert(_find_subset(
                                &targets,
                                sum_key,
                                max_target_length,
                                dp2,
                                Some(&arr_pos),
                                Some(offset),
                            ))
                            .clone(),
                        Err(_) => _find_subset(
                            &targets,
                            sum_key,
                            max_target_length,
                            dp2,
                            Some(&arr_pos),
                            Some(offset),
                        ),
                    }
                };
                if sets.len() == 0 {
                    return;
                }
                sets.dedup();
                let mut keys_removed: Vec<i64> = keys.clone();
                let vec_key: Vec<i64> = i
                    .iter()
                    .enumerate()
                    .map(|(j2, j)| keys_removed.remove(j.0 - j2))
                    .collect();
                sets.iter().for_each(|set| {
                    if set.len() == 0 {
                        return;
                    }
                    let mut keys_for_next = keys_removed.clone();
                    let mut targets_for_next = targets.clone();
                    let mut group_for_next = group.clone();
                    group_for_next.push((vec_key.clone(), set.clone()));
                    set.iter().for_each(|j| {
                        vec_remove(&mut targets_for_next, *j);
                    });
                    sequence_matcher_core(
                        &mut keys_for_next,
                        &mut targets_for_next,
                        &mut group_for_next,
                        &mut answer.clone(),
                        max_key_length,
                        max_target_length,
                        &mut hashmap_fs.clone(),
                        n_candidates,
                        use_all_keys,
                        use_all_targets,
                        max_depth,
                        vec_key.clone(),
                    );
                });
            });
        } else {
            mc.par_bridge().for_each(|i| {
                let sum_key: i64 = i.par_iter().map(|j| j.1).sum();
                if sum_key < last_key.iter().sum() && i.len() == last_key.len() {
                    return;
                }
                let mut sets = {
                    match hashmap_fs.try_write() {
                        Ok(mut v) => v
                            .entry((targets.clone(), sum_key))
                            .or_insert(_find_subset(
                                &targets,
                                sum_key,
                                max_target_length,
                                dp2,
                                Some(&arr_pos),
                                Some(offset),
                            ))
                            .clone(),
                        Err(_) => _find_subset(
                            &targets,
                            sum_key,
                            max_target_length,
                            dp2,
                            Some(&arr_pos),
                            Some(offset),
                        ),
                    }
                };
                if sets.len() == 0 {
                    return;
                }
                sets.dedup();
                let mut keys_removed: Vec<i64> = keys.clone();
                let vec_key: Vec<i64> = i
                    .iter()
                    .enumerate()
                    .map(|(j2, j)| keys_removed.remove(j.0 - j2))
                    .collect();
                sets.par_iter().for_each(|set| {
                    if set.len() == 0 {
                        return;
                    }
                    let mut keys_for_next = keys_removed.clone();
                    let mut targets_for_next = targets.clone();
                    let mut group_for_next = group.clone();
                    group_for_next.push((vec_key.clone(), set.clone()));
                    set.iter().for_each(|j| {
                        vec_remove(&mut targets_for_next, *j);
                    });
                    sequence_matcher_core(
                        &mut keys_for_next,
                        &mut targets_for_next,
                        &mut group_for_next,
                        &mut answer.clone(),
                        max_key_length,
                        max_target_length,
                        &mut hashmap_fs.clone(),
                        n_candidates,
                        use_all_keys,
                        use_all_targets,
                        max_depth,
                        vec_key.clone(),
                    );
                });
            });
        }
        if use_all_keys == false && use_all_targets == false {
            if group.len() == 0 {
                return;
            }
            let elem = AnswerElement {
                answer_arr: group.clone(),
                keys_remainder: keys.clone(),
                targets_remainder: targets.clone(),
            };
            answer.write().unwrap().push(elem.clone());
        }
        ()
    }

    #[test]
    fn test_sequence_matcher() {
        let answer = sequence_matcher(
            &mut vec![-950, 10000],
            &mut vec![5000, 4000, 50],
            10,
            10,
            2,
            true,
            true,
        )
        .unwrap();
        assert_eq!(
            answer[0].answer_arr,
            vec![(vec![-950, 10000], vec![50, 4000, 5000]),]
        );

        let answer = sequence_matcher(
            &mut vec![6, 7, 3, 2, -9, -3, 8, 3, 6, -10],
            &mut vec![3, 2, -6, -8, 2, -9, 0, -5, -3, 37],
            7,
            6,
            30,
            true,
            true,
        )
        .unwrap();
        assert!(answer.len() >= 29 && answer.len() <= 31);
        // Sometimes the answer is 29 or 31, It may be due to parallel execution. Currently I ignore it.

        let answer = sequence_matcher(
            &mut vec![100, 200, 300, 400, 500, 600, -700, 800, 900, 1000],
            &mut vec![300, 700, 500, 600, -700, 2700],
            3,
            2,
            200,
            true,
            true,
        )
        .unwrap();
        assert_eq!(answer.len(), 195);
        assert_eq!(
            answer[0].answer_arr,
            vec![
                (vec![-700], vec![-700]),
                (vec![100, 200], vec![300]),
                (vec![300, 400], vec![700]),
                (vec![500, 600], vec![500, 600]),
                (vec![800, 900, 1000], vec![2700]),
            ]
        );

        let answer = sequence_matcher(
            &mut vec![9, 0, 1, 7, 1],
            &mut vec![7, 2, 8, 0, 1],
            3,
            2,
            100,
            true,
            true,
        )
        .unwrap();
        assert_eq!(answer.len(), 24);
        assert_eq!(
            answer[0].answer_arr,
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
            200,
            true,
            true,
        )
        .unwrap();
        assert_eq!(answer.len(), 5);
        assert_eq!(
            answer[0].answer_arr,
            vec![
                (vec![5, 10], vec![4, 11]),
                (vec![123, 150], vec![273]),
                (vec![1000, 1100], vec![2100]),
            ]
        );
        assert_eq!(
            answer[2].answer_arr,
            vec![(vec![5, 10, 123, 150, 1000, 1100], vec![4, 11, 273, 2100]),]
        );

        let answer = sequence_matcher(
            &mut vec![1, 2, 3],
            &mut vec![1000, 1200],
            10,
            10,
            2,
            true,
            true,
        );
        let expected = Err(format!("The sums of two arrays must be the same values. key's sum is {}. target's sum is {}. The difference is {}.", 6, 2200, -2194));
        assert_eq!(answer, expected);

        let answer = sequence_matcher(
            &mut vec![1, 2, 3, 4],
            &mut vec![1, 5],
            10,
            10,
            2,
            true,
            true,
        );
        let expected = Err(format!("The sums of two arrays must be the same values. key's sum is {}. target's sum is {}. The difference is {}.", 10, 6, 4));

        assert_eq!(answer, expected);

        let answer = sequence_matcher(
            &mut vec![183, 36, 231, 128, 137],
            &mut vec![8, 9, 15, 15, 33, 36, 39, 45, 46, 60, 68, 73, 80, 92, 96],
            1,
            15,
            20,
            true,
            true,
        )
        .unwrap();

        assert_eq!(answer.len(), 13);
        assert_eq!(
            answer[0].answer_arr,
            vec![
                (vec![36], vec![36]),
                (vec![128], vec![9, 39, 80]),
                (vec![137], vec![8, 33, 96]),
                (vec![183], vec![45, 46, 92]),
                (vec![231], vec![15, 15, 60, 68, 73]),
            ]
        );
    }
    #[test]

    fn test_sequence_matcher_allowing_imcomplete_answer() {
        let answer = sequence_matcher(
            &mut vec![1, 2, -3, 4, 5],
            &mut vec![1, 2],
            6,
            4,
            200,
            true,
            false,
        )
        .unwrap();
        assert_eq!(answer.len(), 0);

        let answer = sequence_matcher(
            &mut vec![1, 2, -3, 4, 5],
            &mut vec![1, 2],
            6,
            4,
            200,
            false,
            true,
        )
        .unwrap();
        assert_eq!(answer.len(), 6);
        assert_eq!(answer[0].answer_arr, vec![(vec![-3, 1, 5], vec![1, 2]),]);
    }

    #[test]
    fn test_sequence_matcher_allowing_imcomplete_answer_complicated() {
        let answer = sequence_matcher(
            &mut vec![-755, 222, 851, 291, -511, -567, 958, 939, -28],
            &mut vec![-590, 785, -597, -184, -968, -555, -221, -28],
            30,
            30,
            200,
            false,
            true,
        )
        .unwrap();
        assert_eq!(answer.len(), 0);

        let answer = sequence_matcher(
            &mut vec![-755, 222, 851, 291, -511, -567, 958, 939, -28],
            &mut vec![-590, 785, -597, -184, -968, -555, -221, -28],
            30,
            30,
            100,
            false,
            false,
        )
        .unwrap();
        assert_eq!(
            answer[0].answer_arr,
            vec![(
                vec![-755, -567, -511, -28, 939],
                vec![-968, -555, -184, 785]
            ),]
        );
    }

    #[test]
    fn test_sequence_matcher_huge_values() {
        let answer = sequence_matcher(
            &mut vec![100000, 200000, 300000, 400000, 500000, 600000, -700000, 800000, 900000, 1000000],
            &mut vec![300000, 700000, 500000, 600000, -700000, 2700000],
            3,
            2,
            200,
            true,
            true,
        )
        .unwrap();
        assert_eq!(answer.len(), 195);
        assert_eq!(
            answer[0].answer_arr,
            vec![
                (vec![-700000], vec![-700000]),
                (vec![100000, 200000], vec![300000]),
                (vec![300000, 400000], vec![700000]),
                (vec![500000, 600000], vec![500000, 600000]),
                (vec![800000, 900000, 1000000], vec![2700000]),
            ]
        );
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_subset() {
        let result = dp::find_subset(vec![1, 2, 3], 3, 2);
        let route1: Vec<i64> = vec![3];
        let route2: Vec<i64> = vec![1, 2];
        let answer: Vec<Vec<i64>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset(vec![0, 3, 5, 10], 3, 2);
        let route1: Vec<i64> = vec![3];
        let route2: Vec<i64> = vec![0, 3];
        let answer: Vec<Vec<i64>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset(vec![1, 2, 3, 0], 3, 3);
        let route1: Vec<i64> = vec![3];
        let route2: Vec<i64> = vec![0, 3];
        let route3: Vec<i64> = vec![1, 2];
        let route4: Vec<i64> = vec![0, 1, 2];
        let answer: Vec<Vec<i64>> = vec![route1, route2, route3, route4];
        assert_eq!(result, answer);

        let result = dp::find_subset(vec![1, 2, 3], 3, 2);
        let route1: Vec<i64> = vec![3];
        let route2: Vec<i64> = vec![1, 2];
        let answer: Vec<Vec<i64>> = vec![route1, route2];
        assert_eq!(result, answer);

        let result = dp::find_subset(vec![1, 2, 3, 4, 5], 10, 4);
        let route1: Vec<i64> = vec![2, 3, 5];
        let route2: Vec<i64> = vec![1, 4, 5];
        let route3: Vec<i64> = vec![1, 2, 3, 4];
        let answer: Vec<Vec<i64>> = vec![route1, route2, route3];
        assert_eq!(result, answer);

        let result = dp::find_subset(vec![1, 2, 3, 4, 5], 10, 3);
        let route2: Vec<i64> = vec![2, 3, 5];
        let route3: Vec<i64> = vec![1, 4, 5];
        let answer: Vec<Vec<i64>> = vec![route2, route3];
        assert_eq!(result, answer);

        let arr = vec![75, 467, 512, -835, 770, -69, 10];
        let result = dp::find_subset(arr, 711, 3);
        let route1: Vec<i64> = vec![-69, 10, 770];
        let answer: Vec<Vec<i64>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-3, 10, 56, -33, 65, -9, 8, 72, 63, 35];
        let result = dp::find_subset(arr, 7, 4);
        let route1: Vec<i64> = vec![-3, 10];
        let route2: Vec<i64> = vec![-33, -3, 8, 35];
        let answer: Vec<Vec<i64>> = vec![route1, route2];
        assert_eq!(result, answer);

        let arr = vec![
            73209, 95597, 84735, 40496, 83553, 95595, -628, 201, 27597, 7904, 98445, 6241, 33002,
            -776, -711, 45552, 86746, 84248, 66278, 37475,
        ];
        let result = dp::find_subset(arr, 72782, 3);
        let route1: Vec<i64> = vec![-628, 201, 73209];
        let answer: Vec<Vec<i64>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-1, 2, 3];
        let result = dp::find_subset(arr, -1, 1);
        let route1: Vec<i64> = vec![-1];
        let answer: Vec<Vec<i64>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-10, 5, -2];
        let result = dp::find_subset(arr, -5, 2);
        let route1: Vec<i64> = vec![-10, 5];
        let answer: Vec<Vec<i64>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-3, -5, -7];
        let result = dp::find_subset(arr, -15, 3);
        let route1: Vec<i64> = vec![-7, -5, -3];
        let answer: Vec<Vec<i64>> = vec![route1];
        assert_eq!(result, answer);

        let arr = vec![-100, 10, 20];
        let result = dp::find_subset(arr, -70, 3);
        let route1: Vec<i64> = vec![-100, 10, 20];
        let answer: Vec<Vec<i64>> = vec![route1];
        assert_eq!(result, answer);
    }

    #[test]
    fn test_find_subset_long() {
        let result = dp::find_subset(vec![9999999999, 150, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            , 150, 5);
        let route: Vec<i64> = vec![150];
        let route1: Vec<i64> = vec![0, 150];
        let answer: Vec<Vec<i64>> = vec![route, route1];
        assert_eq!(result, answer);
    }
}
