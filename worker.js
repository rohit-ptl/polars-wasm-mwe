
import * as rustWasm from "../rust-wasm/pkg/rust_wasm.js"

async function start() {
    await rustWasm.default()
    await rustWasm.initThreadPool(navigator.hardwareConcurrency);
    await rustWasm.init_hooks()
}


onmessage = async function (e) {
    var data = JSON.parse(e.data);
    console.log(data);
    let out = rustWasm.process_file(Object.values(data.file), data.learningRate, data.lambda, data.numIter, data.regType);
    this.postMessage(out)
}

await start()
console.log("started worker")