use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = altv_imports)]
    fn log(string: &str);
}

#[wasm_bindgen]
pub fn call_rust_wasm() {
    log("Hello, alt:V!🤩🤯🥶😱");
}
