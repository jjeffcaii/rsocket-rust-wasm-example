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

#[wasm_bindgen]
pub async fn example_request_response(data: String) {
    let cli = RSocketFactory::connect()
        .transport(WebsocketClientTransport::from("ws://127.0.0.1:7878"))
        .start()
        .await
        .expect("Connect failed!");
    let res = cli
        .request_response(Payload::builder().set_data_utf8(&data).build())
        .await
        .expect("Request failed!");
    alert!("request: {}\nresponse: {}", data, res.data_utf8().unwrap());
}
