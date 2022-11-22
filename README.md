

### How to run this repo

- In the rust-wasm folder, run the following command:
`wasm-pack build --target web`
- This will create a pkg folder with the compiled Wasm file and a js file that will be used to import the wasm file into the html file.
- In the root directory we run `python server.py` to start a http server to host the index.html file.
- This will start a server with the right CORS settings to have the shared array buffer working.
- Go to localhost:8000 in your browser. Localhost is important here otherwise there will be some issues with the wasm

### What is needed from this project

- Currently this repo loads a dataframe selected by the user into a polars dataframe (https://github.com/gitkwr/polars)
- The goal here is to use LightGBM to use the data in this polars dataframe to fit a gradient boosting tree
- The current setup (Rust, Polars (uses arrow data format), wasm-bindgen etc..) is not changable (this is only a test repo, we have substantial code using this setup in our main repo and it cannot be changed, e.g., we cannot use wasm-emscripten as a target for rust code)
- You will figure out a way to compile LightGBM (https://github.com/microsoft/LightGBM) to wasm and connect it to existing code such that it takes the polars dataframe and returns predictions
- We cannot have multiple wasm memory buffers
