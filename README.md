# Bevy Gcode viewer

A Gcode viewer with Rust and Bevy.

[Demo](https://imgur.com/NppoVPF)


Run it:

```sh
cargo run --release --features native
```


the wasm version does not work well at all (Camera controls and some problems on Firefox).

```sh
cargo build --release --target wasm32-unknown-unknown --features web

wasm-bindgen --out-dir target/  --out-name wasm --target web --no-typescript target/wasm32-unknown-unknown/release/gcode-pruebas.wasm

basic-http-server -x
```

```sh
cargo build  --target wasm32-unknown-unknown --features web

wasm-bindgen.exe --out-dir target/  --out-name wasm --target web --no-typescript target/wasm32-unknown-unknown/debug/gcode-pruebas.wasm

basic-http-server -x
```