# go-engine (Rust WASM)

Rudimentary Tromp–Taylor Go rules engine compiled to WebAssembly.

- Move legality, captures, suicide prevention
- Ko/repetition prevention via Zobrist hashing
- Pass, resign, undo/redo, scoring, group+liberties

Build locally (inside devcontainer):

```
# install wasm-pack if needed
cargo install wasm-pack --locked

# build
wasm-pack build --target web --release

# copy to frontend
mkdir -p ../src/lib/go-wasm
cp pkg/* ../src/lib/go-wasm/
```

In Svelte, dynamically import `src/lib/go-wasm/go_engine.js` and use `new Game(19)`.
