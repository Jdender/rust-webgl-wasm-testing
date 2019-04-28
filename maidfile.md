## build

Builds everything in the workspace.

Run task `build:client` after this.

```bash
cargo build
```

## build:client

Uses wasm-pack to build the client to ./client/pkg

```bash
wasm-pack build --target web client
```

## serve

Serve the client to localhost.

```bash
basic-http-server -a 127.0.0.1:8080 client
```
