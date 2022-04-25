mod window;
pub mod document;
pub mod console;
mod observable;

pub(crate) use crate::console::console_log;

use wasm_bindgen::prelude::*;
use crate::window::window;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Document {
    title: String,
}

struct Window {

}

impl Window {

}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let window = window();
    window.document().title = name.to_string()
    // let body = document.body().expect("document should have a body");
    // alert(&format!("Hello, {}!", name));
}