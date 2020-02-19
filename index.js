// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const rust = import('./pkg/rsocket_rust_wasm_example');

const encoder = new TextEncoder();
const decoder = new TextDecoder();

let data = encoder.encode("ping");

rust
    .then(m => {
        say('connecting...');

        m.connect("ws://127.0.0.1:7878")
            .then(cli => {
                cli.request_response(m.new_payload("hello @" + new Date().toString() + "!"))
                    .then(res => {
                        say(payload2string(res));
                    }).catch(console.error);

                document.getElementById('btn_request_response').addEventListener('click', () => {
                    cli.request_response(new m.new_payload("hello @" + new Date().toString() + "!"))
                        .then(res => {
                            say(payload2string(res));
                        }).catch(console.error);
                });
                say('connect success!');
            })
            .catch(console.error);
    })
    .catch(console.error);

function payload2string(payload) {
    let d = undefined;
    let m = undefined;
    if (payload.data) {
        let foo = new Uint8Array(payload.data);
        d = decoder.decode(foo);
    }
    if (payload.metadata) {
        let foo = new Uint8Array(payload.metadata);
        m = decoder.decode(foo);
    }
    return `Payload{ data=${d}, metadata=${m} }`;
}

function say(input) {
    var x = document.createElement("p");
    var t = document.createTextNode(input);
    x.appendChild(t);
    document.getElementById('msgbox').appendChild(x);
}
