use rsocket_rust::prelude::*;
use rsocket_rust_transport_wasm::*;
use wasm_bindgen::prelude::*;

macro_rules! alert {
    ($($t:tt)*) => (alert(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
