const worker = new Worker("./worker.js", { type: "module" });

worker.addEventListener("message", async ({ data }) => {
    document.getElementById("output").innerHTML = data;
});

worker.onerror = function (event) {
    throw new Error("s" + event.message + " (" + event.filename + ":" + event.lineno + ")");
}

// define a function to send a message to the worker
window.sendMessage = function (args) {
    console.log("Sending message to worker");
    worker.postMessage(args);
};


async function loadFile() {
    var files = document.getElementById('file').files;
    var file = files[0];
    // check if file was selected
    if (!file) {
        alert("Please select a file");
        return;
    }

    // send the file to the worker
    sendMessage(file)
}

document.getElementById("upload").addEventListener("click", loadFile, false);