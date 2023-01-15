use polars::prelude::{CsvEncoding, CsvReader, Float64Type, SerReader};
use polars::series::Series;
use std::usize;
use std::{io::Cursor, panic};

// export the function to JavaScript
use ndarray::{concatenate, s, Array1, Array2};
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use wasm_logger;

fn l2_norm(array: Array1<f64>) -> f64 {
    array.dot(&array).sqrt()
}

fn logistic_regression(X: &Array2<f64>, beta: &Array1<f64>) -> Array1<f64> {
    if X.shape()[1] < beta.shape()[0] {
        // Add an additional column to X to act as the bias parameter
        let new_col = Array1::<f64>::ones(X.shape()[0]);
        let mut X = X.clone();
        X.push_column(new_col.view());
        let z = X.dot(beta);
        log::info!("z:: {}", z);
        return z.mapv(|x| 1.0 / (1.0 + (-x).exp()));
    } else {
        let z = X.dot(beta);
        z.mapv(|x| 1.0 / (1.0 + (-x).exp()))
    }
}

#[derive(Clone, Copy)]
enum RegularisationOptions {
    L1,
    L2,
}

fn logistic_regression_parameter_optimisation(
    X: &Array2<f64>,
    y: &Array1<f64>,
    max_iter: usize,
    tol: f64,
    learning_rate: f64,
    regularisation: Option<RegularisationOptions>,
    lambda: Option<f64>,
) -> Array1<f64> {
    // Initialise weights to zero, with them being the same size as the nubmer of input features
    // plus an extra (bias)
    let mut beta = Array1::zeros(X.shape()[1] + 1);
    let m = X.shape()[1] as f64;

    // Add an additional column to X to act as the bias parameter
    let new_col = Array1::<f64>::ones(X.shape()[0]);
    let mut X = X.clone();
    X.push_column(new_col.view());

    for _ in 0..max_iter {
        let w = logistic_regression(&X, &beta);

        // Calculate the gradient of the weights with respect to the loss function
        // let gradient = &X.t().dot(&(w - y));

        let gradient = &X.t().dot(&(w - y));

        let gradient = match regularisation {
            Some(reg) => match reg {
                RegularisationOptions::L1 => (gradient + lambda.unwrap() * beta.mapv(|x| x.signum())) * 1.0/m,
                RegularisationOptions::L2 => (gradient + lambda.unwrap() * &beta)*1.0/m,
            },
            None => gradient.clone(),
        };

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

fn accuracy(y_pred: &Array1<f64>, y_true: &Array1<f64>) -> f64 {
    let total_correct = y_pred
        .iter()
        .zip(y_true.iter())
        .map(|(x, y)| {
            if x.round() as usize == *y as usize {
                1.0
            } else {
                0.0
            }
        })
        .sum::<f64>();
    let accuracy = total_correct / y_pred.shape()[0] as f64;
    return accuracy;
}

fn f1_score(y_pred: &Array1<f64>, y_true: &Array1<f64>) -> f64 {
    let tp = y_pred
        .iter()
        .zip(y_true.iter())
        .filter(|(x, y)| x.round() == y.round() && x.round() == 1.0)
        .count();
    let fp = y_pred
        .iter()
        .zip(y_true.iter())
        .filter(|(x, y)| x.round() != y.round() && x.round() == 1.0)
        .count();
    let fn_ = y_pred
        .iter()
        .zip(y_true.iter())
        .filter(|(x, y)| x.round() != y.round() && x.round() == 0.0)
        .count();
    let precision = tp as f64 / (tp as f64 + fp as f64);
    let recall = tp as f64 / (tp as f64 + fn_ as f64);
    2.0 * (precision * recall) / (precision + recall)
}

#[wasm_bindgen]
pub fn init_hooks() {
    // better error messages
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn process_file(
    buffer: &[u8],
    learning_rate: f64,
    lambda: f64,
    max_iter: usize,
    int_regularisation: usize,
) -> String {
    log::info!(
        "max_iter: {}, learning_rate: {}, regularisation: {}, lambda: {}",
        max_iter,
        learning_rate,
        int_regularisation,
        lambda
    );

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

    let X = dataframe
        .select(&feature_columns)
        .unwrap()
        .to_ndarray::<Float64Type>()
        .unwrap();
    let y_true = dataframe
        .select(vec![&target_column])
        .unwrap()
        .to_ndarray::<Float64Type>()
        .unwrap()
        .into_shape((X.shape()[0]))
        .unwrap();

    let tol = 0.01;

    let regularisation = match int_regularisation {
        1 => RegularisationOptions::L1,
        2 => RegularisationOptions::L2,
        _ => panic!("Forgot ya regularisation"),
    };

    let beta = logistic_regression_parameter_optimisation(
        &X,
        &y_true,
        max_iter,
        tol,
        learning_rate,
        Some(regularisation),
        Some(lambda),
    );

    let y_pred = logistic_regression(&X, &beta);
    log::debug!("THe predictions are {:?}", y_pred);

    let accuracy = accuracy(&y_pred, &y_true);

    log::debug!("Our optimised parameters are {:?}", beta);

    log::debug!("The predictions are {:?}", &y_pred.slice(s![0..20]));
    log::debug!("The true values are {:?}", &y_true.slice(s![0..20]));
    log::debug!("The accuracy is {:?}", &accuracy);
    log::debug!("The F1 is {}", f1_score(&y_pred, &y_true));
    
    /*
    for foo in y_pred.iter() {
        log::debug!("{}", foo);
    }
    */

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
