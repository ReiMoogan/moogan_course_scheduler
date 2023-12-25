use parse::CourseListContext;
use solver::{BTSolver, CoursePreferences};
use serde_json::Value;
use wasm_bindgen::prelude::*;

mod parse;
mod solver;
mod utils;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn solve(gql_response_js_obj: JsValue, want: Vec<u64>) -> JsValue {
    
    let serde_gql_response: Value = serde_wasm_bindgen::from_value(gql_response_js_obj).unwrap();
    // serde_wasm_bindgen::to_value(&vec![vec![SampleStruct::default()]]).unwrap()
    // vec![SampleStruct::default(), SampleStruct::default()]
    // vec![vec![SampleStruct::default(), SampleStruct::default()]]
    // let res = fs::read("data/mess.json");
    // let gql_response = serde_json::from_str(std::str::from_utf8(&res.unwrap()).unwrap()).unwrap();
    
    // let gql_response_json: Value = serde_json::from_str(&r).unwrap();
    let ctx = match CourseListContext::new(&serde_gql_response) {
        Ok(ctx) => ctx,
        Err(err) => return serde_wasm_bindgen::to_value(err.msg).unwrap()
    };

    let prefs = match CoursePreferences::new(want, ctx) {
        Ok(prefs) => prefs,
        Err(err) => return serde_wasm_bindgen::to_value(err.msg).unwrap()
    };

    let solver = BTSolver::new(prefs);
    let res = solver.solve();
    
    serde_wasm_bindgen::to_value(&res).unwrap()
}