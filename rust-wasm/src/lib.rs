use polars::prelude::{CsvEncoding, CsvReader, SerReader};
use std::{io::Cursor, panic};
use wasm_bindgen::prelude::wasm_bindgen;

// export the function to JavaScript
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn init_hooks() {
    // better error messages
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn process_file(buffer: &[u8]) -> String {
    let mut output = String::new();

    let cursor = Cursor::new(buffer);
    let dataframe = CsvReader::new(cursor)
        .has_header(true)
        .with_chunk_size(1000)
        .with_encoding(CsvEncoding::Utf8)
        .low_memory(true)
        .finish()
        .unwrap();
    let dtypes = dataframe.dtypes();
    let columns = dataframe.get_column_names_owned();

    output.push_str("TOP 10 ROWS\n\n");

    for (header, dtype) in columns.iter().zip(dtypes.iter()) {
        output.push_str(&header);
        output.push_str(": ");
        output.push_str(&dtype.to_string());
        output.push_str(",");
    }

    output.push_str("\r\n");
    for i in 0..10 {
        let row = dataframe.get_row(i);

        for j in row.0.iter() {
            output.push_str(&j.to_string());
            output.push_str(",");
        }
        output.push_str("\r\n");
    }

    output
}
