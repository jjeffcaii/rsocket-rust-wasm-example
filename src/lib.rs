use rsocket_rust::prelude::*;
use rsocket_rust_transport_wasm::{WASMSpawner, WebsocketClientTransport};
use wasm_bindgen::prelude::*;

macro_rules! alert {
    ($($t:tt)*) => (alert(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//"wss://echo.websocket.org"
#[wasm_bindgen]
pub fn start_websocket(url: &str) -> Result<(), JsValue> {
    let url = url.to_owned();
    WASMSpawner.spawn(async move {
        let cli = RSocketFactory::connect()
            .transport(WebsocketClientTransport::from(&url))
            .setup(Payload::from("Hello WASM!"))
            .start_with_runtime(WASMSpawner) // <-- you must use WASMSpawner
            .await
            .unwrap();
        let res: Payload = cli.request_response(Payload::from("Ping")).await.unwrap();
        let (data, _) = res.split();
        alert!("{}", String::from_utf8(data.unwrap().to_vec()).unwrap());
    });
    Ok(())
}
