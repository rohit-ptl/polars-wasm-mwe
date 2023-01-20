

### How to run this repo

- This repo is a minimum working example of rust-polars with WASM
- In the rust-wasm folder, run the following command:
`wasm-pack build --target web`
- This will create a pkg folder with the compiled Wasm file and a js file that will be used to import the wasm file into the html file.
- In the root directory we run `python server.py` to start a http server to host the index.html file.
- This will start a server with the right CORS settings to have the shared array buffer working.
- Go to localhost:8000 in your browser. Localhost is important here otherwise there will be some CORS issues with the default 0.0.0.0
