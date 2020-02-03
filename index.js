// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const rust = import('./pkg/hello_rust_wasm');

rust
    .then(m => {
        console.log('***** BEGIN *****');
        m.start_websocket("ws://127.0.0.1:7878/");
    })
    .catch(console.error);

