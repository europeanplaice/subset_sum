mod dp_module;

// #[cfg(feature = "python")]
mod py_module;

pub use self::dp_module::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_find_subset(
    keys: String,
    targets: String,
    max_key_length: usize,
    max_target_length: usize,
    n_candidates: usize,
    use_all_keys: bool,
    use_all_targets: bool,
) -> String {
    let mut keys: Vec<i32> = keys
        .split("\n")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    if targets.contains("\n") {
        let mut targets: Vec<i32> = targets
            .split("\n")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        let result: Vec<dp::AnswerElement> = match dp::sequence_matcher(
            &mut keys,
            &mut targets,
            max_key_length,
            max_target_length,
            n_candidates,
            use_all_keys,
            use_all_targets,
        ) {
            Ok(res) => res,
            Err(err) => return err,
        };
        if result.len() == 0 {
            return "No solution. You might want to increase maximum length.".to_string();
        }
        dp::sequence_matcher_formatter(result)
    } else {
        let res: Vec<Vec<i32>> =
            dp::find_subset(keys, targets.parse::<i32>().unwrap(), max_target_length);
        let mut r3: Vec<String> = vec![];
        if res.len() == 0 {
            return "No solution. You might want to increase maximum subset length.".to_string();
        }
        for r in res {
            let mut r2: String = r
                .into_iter()
                .map(|x| format!("{}, ", x.to_string()))
                .collect::<String>();
            r2 = format!("[{}]", r2).replace(", ]", "]");
            r3.push(r2);
        }
        let s: String = r3.join(", ");
        s
    }
}
