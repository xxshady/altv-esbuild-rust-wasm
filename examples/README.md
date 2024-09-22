# examples

`shared/main.js` contains JS code that will be executed on alt:V server or client

`shared/rust-wasm` contains Rust crate

## How to run

First of all, run `npm install` in shared directory, also make sure you have installed [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack).

Navigate to altv-server directory and run:

1. `npm install`
2. `npx altv-pkg release` (or other alt:V branch)

### clientside

Navigate to clientside directory and run `npm run build`, after that you can run alt:V server in altv-server directory

### serverside

Navigate to serverside directory and run `npm run build`, after that you can run alt:V server in altv-server directory
