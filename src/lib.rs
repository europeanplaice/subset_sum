
mod dp_module;

#[cfg(feature="python")]
mod py_module;

pub use self::dp_module::*;

#[cfg(feature="wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature="wasm")]
#[wasm_bindgen]
pub fn wasm_find_subset(arr: String, value: i32, max_length: usize) -> String {
    let arr: Vec<i32> = arr.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let res: Vec<Vec<i32>> = dp::find_subset(arr, value, max_length);
    let mut r3: Vec<String> = vec![];
    for r in res{
        let mut r2: String = r.into_iter().map(|x| format!("{}, ", x.to_string())).collect::<String>();
        r2 = format!("[{}]", r2).replace(", ]", "]");
        r3.push(r2);
    }
    let s: String = r3.join(", ");
    s
}