mod canvas;
mod context;
mod path;

extern crate painter_core;
extern crate painter_font;

extern crate wee_alloc;
extern crate js_sys;
extern crate web_sys;
extern crate wasm_bindgen;
extern crate base64;

use crate::wasm_bindgen::prelude::wasm_bindgen;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "getDate")]
    fn performance() -> f64;
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    fn js_console_info(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn js_console_error(s: &str);
}








