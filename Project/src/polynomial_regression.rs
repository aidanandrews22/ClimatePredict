//////////////////////////////////////// dependencies ////////////////////////////////////////

use nalgebra::{DMatrix, DVector};
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io;

//////////////////////////////////////// global variables ////////////////////////////////////////

const DATA: &str = "./data/emission_temp_data.csv";
const TEST_RATIO: f64 = 0.2; // Percentage of data to be used for testing

//////////////////////////////////////// helper functions ////////////////////////////////////////

/// Reads data from a CSV file and parses time and emissions into vectors.
fn read_data(path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(File::open(path)?);
    let mut times = Vec::new();
    let mut emissions = Vec::new();

    for result in rdr.records() {
        let record = result?;
        times.push(record[0].parse::<f64>()?);
        emissions.push(record[1].parse::<f64>()?);
    }

    Ok((times, emissions))
}

/// Splits data into training and testing sets based on a specified ratio.
/// 
/// Args:
///     x: Time data as a slice of f64.
///     y: Emissions data as a slice of f64.
///     ratio: Fraction of data to use as the test set.
/// 
/// Returns:
///     Training and testing datasets as matrices.
fn split_data(x: &[f64], y: &[f64], ratio: f64) -> ((DMatrix<f64>, DMatrix<f64>), (DMatrix<f64>, DMatrix<f64>)) {
    let test_size = (x.len() as f64 * ratio).round() as usize;
    let train_size = x.len() - test_size;

    let x_train = DMatrix::from_column_slice(train_size, 1, &x[..train_size]);
    let y_train = DMatrix::from_column_slice(train_size, 1, &y[..train_size]);
    let x_test = DMatrix::from_column_slice(test_size, 1, &x[train_size..]);
    let y_test = DMatrix::from_column_slice(test_size, 1, &y[train_size..]);

    ((x_train, y_train), (x_test, y_test))
}

/// Builds a matrix of polynomial features from a single variable input.
/// 
/// Args:
///     x: Input data as a DMatrix<f64>.
///     degree: The degree of polynomial features to generate.
/// 
/// Returns:
///     A matrix where each column represents x to the power of the column index.
fn build_polynomial_features(x: &DMatrix<f64>, degree: usize) -> DMatrix<f64> {
    let n = x.nrows();
    let mut x_poly = DMatrix::from_element(n, degree + 1, 1.0);

    for i in 1..=degree {
        for j in 0..n {
            x_poly[(j, i)] = x_poly[(j, i - 1)] * x[(j, 0)];
        }
    }

    x_poly
}

/// Performs polynomial regression using the normal equation and Cholesky decomposition.
/// 
/// Args:
///     x: Independent variable (time) as a DMatrix.
///     y: Dependent variable (emissions) as a DMatrix.
///     degree: Degree of the polynomial.
/// 
/// Returns:
///     Vector of coefficients for the polynomial model.
fn polynomial_regression(x: &DMatrix<f64>, y: &DMatrix<f64>, degree: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    // Builds the design matrix of polynomial features
    let x_poly = build_polynomial_features(x, degree);
    
    // Compute the transpose of the polynomial feature matrix
    let xt = x_poly.transpose();
    
    // Perform matrix multiplication for X^T * X and X^T * y
    let xt_x = &xt * &x_poly;
    let xt_y = xt * y;

    // Solves the normal equations using Cholesky decomposition for linear least squares
    let chol = nalgebra::linalg::Cholesky::new(xt_x).ok_or("Cholesky decomposition failed")?;
    let beta = chol.solve(&xt_y);

    // Extract the coefficients from the solution matrix
    Ok(beta.column(0).iter().cloned().collect())
}

//////////////////////////////////////// main function ////////////////////////////////////////

fn main() -> Result<(), Box<dyn Error>> {
    // Load data from the CSV file
    let (times, emissions) = read_data(DATA)?;
    
    // Split data into training and testing sets
    let ((x_train, y_train), (x_test, y_test)) = split_data(&times, &emissions, TEST_RATIO);

    // Train the polynomial regression model to find coefficients
    let degree = 3;
    let coefficients = polynomial_regression(&x_train, &y_train, degree)?;

    // Validate the model using the test set and compute mean squared error
    let x_test_poly = build_polynomial_features(&x_test, degree);
    let predictions = x_test_poly * DVector::from_column_slice(&coefficients);
    let mse = mean_squared_error(predictions.column(0).as_slice(), y_test.column(0).as_slice());

    // Print the model's performance and coefficients
    println!("Mean Squared Error on Test Set: {:.3}", mse);
    println!("Model trained with polynomial coefficients:");
    for (i, coeff) in coefficients.iter().enumerate() {
        println!("Coefficient a_{} = {:.4}", i, coeff);
    }

    loop {
        println!("Enter time value (year) or type 'exit' to quit:");
        let mut time_input = String::new();
        io::stdin().read_line(&mut time_input)?;
        if time_input.trim().eq("exit") {
            break;
        }

        let mut time_value: f64 = time_input.trim().parse().unwrap_or_else(|_| {
            println!("Please enter a valid number.");
            0.0
        });
        time_value = time_value + 2023.0;
        let mut predicted_emission = 0.0;
        for (i, &coeff) in coefficients.iter().enumerate() {
            predicted_emission += coeff * time_value.powi(i as i32);
        }
        println!("Predicted emission: {:.2} GtCO₂", predicted_emission);
    }

    Ok(())
}

/// Computes the Mean Squared Error (MSE) between predicted values and actual values.
/// 
/// Args:
///     predictions: Predicted values as a slice of f64.
///     targets: Actual values as a slice of f64.
/// 
/// Returns:
///     The mean squared error as a f64.
fn mean_squared_error(predictions: &[f64], targets: &[f64]) -> f64 {
    predictions.iter().zip(targets.iter())
        .map(|(p, t)| (p - t).powi(2))
        .sum::<f64>() / predictions.len() as f64
}

// cargo build --bin polynomial_regression --features cuda
// cargo run --bin polynomial_regression --features cuda