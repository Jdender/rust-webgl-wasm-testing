# Workspace

## build

Builds everything in the workspace.

Run task `client-build` after this.

```bash
cargo build --all --exclude client
```

# Client

## client-build

Uses wasm-pack to build the client to ./client/pkg.

```bash
wasm-pack build --target web client
```

## client-serve

Serve the client to localhost.

```bash
basic-http-server -a 127.0.0.1:8080 client
```

## client-watch

Rerun client:build on any file changes.

```bash
cd client
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build  --target web"
```
