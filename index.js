const worker = new Worker("./worker.js", { type: "module" });

worker.addEventListener("message", async ({ data }) => {
    document.getElementById("output").innerHTML = data;
});

worker.onerror = function(event) {
    throw new Error("s" + event.message + " (" + event.filename + ":" + event.lineno + ")");
}

// define a function to send a message to the worker
window.sendMessage = function(args) {
    console.log("Sending message to worker");
    worker.postMessage(args);
};


async function loadFile(e) {
    e.preventDefault();
    var files = document.getElementById('file').files;
    var file = files[0];
    // check if file was selected

    var learningRate = document.getElementById("learning-rate");
    var lambda = document.getElementById("lambda");
    var numIter = document.getElementById("num-iter");
    var regType = document.getElementById("reg-type");


    if (!file) {
        alert("Please select a file");
        return;
    }

    let buffer = await file.arrayBuffer()
    let arr = new Uint8Array(buffer)
    // send the file to the worker
    sendMessage(JSON.stringify({
        file: arr,
        learningRate: +learningRate.value,
        lambda: +lambda.value,
        numIter: +numIter.value,
        regType: +regType.value
    }));
}


var form = document.getElementById("our-form");
form.onsubmit = loadFile;

// document.getElementById("upload").addEventListener("click", loadFile, false);