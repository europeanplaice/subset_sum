mod dp_module;

#[cfg(feature = "python")]
mod py_module;

pub use self::dp_module::*;

pub mod reconciliation;

#[cfg(feature = "wasm")]
use serde_wasm_bindgen;
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
    let mut keys: Vec<i64> = keys
        .split("\n")
        .filter_map(|x| x.trim().parse::<i64>().ok())
        .collect();
    if targets.contains("\n") {
        let mut targets: Vec<i64> = targets
            .split("\n")
            .filter_map(|x| x.trim().parse::<i64>().ok())
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
        let res: Vec<Vec<i64>> =
            dp::find_subset(keys, targets.parse::<i64>().unwrap(), max_target_length);
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

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_reconcile(
    keys_json: &str,
    targets_json: &str,
    max_key_group_size: usize,
    max_target_group_size: usize,
    tolerance: i64,
    n_candidates: usize,
) -> Result<JsValue, JsValue> {
    let keys: Vec<reconciliation::Transaction> = serde_json::from_str(keys_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse keys JSON: {}", e)))?;
    let targets: Vec<reconciliation::Transaction> = serde_json::from_str(targets_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse targets JSON: {}", e)))?;

    let config = reconciliation::ReconciliationConfig {
        max_key_group_size,
        max_target_group_size,
        tolerance,
        n_candidates,
    };

    match reconciliation::reconcile(keys, targets, config) {
        Ok(result) => {
            let json_result = serde_json::to_string(&result)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))?;
            Ok(JsValue::from_str(&json_result))
        }
        Err(e) => Err(JsValue::from_str(&e)),
    }
}
