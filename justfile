default:
	echo 'Hello, world!'
build:
        @wasm-pack build
        @yarn
serve:
        yarn run serve
