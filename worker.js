
import * as rustWasm from "../rust-wasm/pkg/rust_wasm.js"

async function start() {
    await rustWasm.default()
    await rustWasm.initThreadPool(navigator.hardwareConcurrency);
    await rustWasm.init_hooks()
}


onmessage = async function (e) {
    // receive the file from the main thread
    var file = e.data;
    console.log(file);
    let buffer = await e.data.arrayBuffer()
    console.log(buffer);
    let arr = new Uint8Array(buffer)

    let out = rustWasm.process_file(arr)
    this.postMessage(out)
}

await start()
console.log("started worker")