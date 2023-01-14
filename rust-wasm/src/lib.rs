use polars::prelude::{CsvEncoding, CsvReader, SerReader, Float64Type};
use polars::series::Series;
use std::{io::Cursor, panic};

// export the function to JavaScript
pub use wasm_bindgen_rayon::init_thread_pool;
use wasm_bindgen::prelude::*;
use wasm_logger;
use ndarray::{Array1, Array2, s};

fn l2_norm(array: Array1<f64>) -> f64 {
    array.dot(&array)
}

fn logistic_regression(
    X: &Array2<f64>, 
    beta: &Array1<f64>
    ) -> Array1<f64> {
    let z = X.dot(beta);
    z.mapv(|x| 1.0 / (1.0 + (-x).exp()))
}

fn logistic_regression_parameter_optimisation(
    X: &Array2<f64>, 
    y: &Array1<f64>, 
    max_iter: usize, 
    tol: f64, 
    learning_rate: f64
    ) -> Array1<f64> {

    // Initialise weights to zero, with them being the same size as the nubmer of input features
    let mut beta = Array1::zeros(X.shape()[1]);

    for _ in 0..max_iter {
        let w = logistic_regression(&X, &beta);

        // Calculate the gradient of the weights with respect to the loss function
        let gradient = &X.t().dot(&(w - y));

        // Perform the gradient step
        let beta_new = &beta - learning_rate * gradient;

        // Check if the change to the weights is below the tolerance
        if l2_norm(&beta_new - &beta) < tol {
            break;
        }

        beta = beta_new;
    }
    beta
}

fn accuracy(
    y_pred: &Array1<f64>,
    y_true: &Array1<f64>
    ) -> f64 {
    let total_correct = y_pred.dot(y_true);
    let accuracy = total_correct / y_pred.shape()[0] as f64;
    return accuracy
}


#[wasm_bindgen]
pub fn init_hooks() {
    // better error messages
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::info!("Some info");
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
    
    // Set the target column
    let target_column = "target";

    // Assume the features are the remaining columns
    let feature_columns = dataframe
        .get_columns()
        .iter()
        .filter(|col_name| col_name.name() != target_column)
        .map(|x| x.name().to_string())
        .collect::<Vec<String>>();

    let X = dataframe.select(&feature_columns).unwrap().to_ndarray::<Float64Type>().unwrap();
    let y_true = dataframe.select(vec![&target_column]).unwrap().to_ndarray::<Float64Type>().unwrap().into_shape((X.shape()[0])).unwrap(); 
    let max_iter = 1000;
    let tol = 0.01;
    let learning_rate = 0.1;

    let beta = logistic_regression_parameter_optimisation(&X, &y_true, max_iter, tol, learning_rate);

    let y_pred = logistic_regression(&X, &beta);

    let accuracy = accuracy(&y_pred, &y_true);

    log::debug!("Our optimised parameters are {:?}", beta);

    log::debug!("The predictions are {:?}", &y_pred.slice(s![0..20]));
    log::debug!("The true values are {:?}", &y_true.slice(s![0..20]));
    log::debug!("The accuracy is {:?}", &accuracy);

    // Run the logistic regression
    output.push_str("TOP 10 ROWS\n\n");

    for (header, dtype) in columns.iter().zip(dtypes.iter()) {
        output.push_str(&header);
        output.push_str(": ");
        output.push_str(&dtype.to_string());
        output.push_str(",");
    }

    
    output
}
