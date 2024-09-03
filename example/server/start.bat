cd .. &&^
npm run build-wasm &&^
npm run generate-sourcemap &&^
npm run build-js &&^
cd server &&^
copy /Y ..\dist.js resources\rust\client.js &&^
copy /Y ..\rust-wasm\pkg\rust_wasm_bg.wasm.map resources\rust\wasm.map &&^
cd ..\rust-server &&^
cargo build &&^
cd ../server &&^
copy /Y ..\rust-server\target\debug\rust_server.dll resources\rust\server.dll &&^
altv-server.exe
