//////////////////////////////////////// dependencies ////////////////////////////////////////

use candle::{Device, Tensor};
use csv::ReaderBuilder;
use std::error::Error;
use std::io;

//////////////////////////////////////// global variables ////////////////////////////////////////

const DATA: &str = "./data/emission_temp_data.csv";

//////////////////////////////////////// helper functions ////////////////////////////////////////

fn mean_squared_error(predictions: &[f64], targets: &[f64]) -> f64 {
    predictions.iter().zip(targets.iter())
        .map(|(pred, target)| (pred - target).powi(2))
        .sum::<f64>() / predictions.len() as f64
}


fn process_data(data: &str) -> Result<((Tensor, Tensor), (Vec<f64>, Vec<f64>)), Box<dyn Error>> {
    let file = std::fs::File::open(data)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut emissions = Vec::new();
    let mut temps = Vec::new();

    for result in reader.records() {
        let record = result?;
        emissions.push(record[1].parse::<f64>()?);
        temps.push(record[2].parse::<f64>()?);
    }

    let device = Device::new_cuda(0)?;
    let emissions_tensor = Tensor::from_slice(&emissions, (emissions.len(), 1), &device)?;
    let temps_tensor = Tensor::from_slice(&temps, (temps.len(), 1), &device)?;

    Ok(((emissions_tensor, temps_tensor), (emissions, temps)))
}


fn split_data(emissions: &[f64], temps: &[f64], test_ratio: f64) -> ((Vec<f64>, Vec<f64>), (Vec<f64>, Vec<f64>)) {
    let test_size = (emissions.len() as f64 * test_ratio).round() as usize;
    let training_size = emissions.len() - test_size;

    let training_data = (emissions[..training_size].to_vec(), temps[..training_size].to_vec());
    let test_data = (emissions[training_size..].to_vec(), temps[training_size..].to_vec());

    (training_data, test_data)
}


fn linear_regression(x: &Tensor, y: &Tensor) -> Result<(f64, f64), Box<dyn Error>> {

    // Calculate the mean of the emissions tensor: ∀x ∈ emissions_tensor: (Σx_i + x_i+1) / #of elements
    let x_mean = x.mean(0)?.mean(0)?.to_scalar::<f64>()?;
    
    // Calculate the mean of the temperature tensor: ∀x ∈ temps_tensor: (Σx_i + x_i+1) / #of elements
    let y_mean = y.mean(0)?.mean(0)?.to_scalar::<f64>()?;

    // calculates the difference between each element and the mean, for computing the covariance/variance
    let x_diff = x - x_mean;
    let y_diff = y - y_mean;

    let x_diff_tensor = x_diff?;
    let y_diff_tensor = y_diff?;

    // calculates the sum of the products of differences. This is the covariance between x and y (how much the variables change each other)
    let numerator = (&x_diff_tensor * &y_diff_tensor)?.sum_all()?.to_scalar::<f64>()?;
    // calculates the sum of the squared residuals of x. This is the variance (how spread out the data is)
    let denominator = (&x_diff_tensor * &x_diff_tensor)?.sum_all()?.to_scalar::<f64>()?;
    println!("Numerator: {}, Denominator: {}", numerator, denominator);

    // This is the ratio of the covariance to variance β=cov(x,y)/var(x)
    let slope = numerator / denominator;
    // This is the y intercept of the regression line α = y(mean) - βx(mean)
    let intercept = y_mean - slope * x_mean;

    Ok((slope, intercept))
}

fn test_model(data: &str, test_ratio: f64) -> Result<f64, Box<dyn Error>> {
    let ((emissions_tensor, temps_tensor), (emissions, temps)) = process_data(data)?;

    // Split raw data into training and testing sets
    let ((emissions_train, temps_train), (emissions_test, temps_test)) = split_data(&emissions, &temps, test_ratio);

    // Assuming `device` is already initialized and available
    let device = Device::new_cuda(0)?;

    // Convert training data into tensors
    let emissions_tensor_train = Tensor::from_slice(&emissions_train, (emissions_train.len(), 1), &device)?;
    let temps_tensor_train = Tensor::from_slice(&temps_train, (temps_train.len(), 1), &device)?;

    // Train model using tensors from training data
    let (slope, intercept) = linear_regression(&emissions_tensor_train, &temps_tensor_train)?;

    // Convert test data into tensors
    let emissions_tensor_test = Tensor::from_slice(&emissions_test, (emissions_test.len(), 1), &device)?;
    let temps_tensor_test = Tensor::from_slice(&temps_test, (temps_test.len(), 1), &device)?;

    // Use model to predict temperatures for test data
    let predictions: Vec<f64> = emissions_tensor_test.strided_index()
        .map(|em| slope * em as f64 + intercept)
        .collect();

    // Calculate Mean Squared Error
    let mse = mean_squared_error(&predictions, &temps_test);

    Ok(mse)
}

//////////////////////////////////////// main ////////////////////////////////////////

fn main() -> Result<(), Box<dyn Error>> {
    // Split ratio for the training and test data
    let test_ratio = 0.2;  // For example, use 20% of the data for testing

    // Run the test model function which also trains the model
    let mse = test_model(DATA, test_ratio)?;

    // Print the mean squared error to evaluate the model's performance
    println!("Mean Squared Error on Test Set: {:.3}", mse);

    // Example of using the model interactively to predict temperatures based on emission input
    let ((emissions_tensor, temps_tensor), (_emissions, _temps)) = process_data(DATA)?;
    let (slope, intercept) = linear_regression(&emissions_tensor, &temps_tensor)?;

    println!("Model trained with parameters: Slope (β) = {:.4}, Intercept (α) = {:.4}", slope, intercept);

    loop {
        println!("Enter emission value (g CO2/kWh) or type 'exit' to quit:");
        let mut emission_input = String::new();
        io::stdin().read_line(&mut emission_input)?;
        if emission_input.trim().eq("exit") {
            break;
        }

        let mut emission_value: f64 = emission_input.trim().parse().unwrap_or_else(|_| {
            println!("Please enter a valid number.");
            0.0 // Default value if parse fails; could also choose to re-prompt for input
        });
        emission_value = emission_value + 40.9;
        // Calculate and print the predicted temperature change based on the input
        let mut predicted_temp = slope * emission_value + intercept;
        predicted_temp = predicted_temp - 1.01;
        println!("Predicted temperature change: {:.2} °C", predicted_temp);
    }

    Ok(())
}

// cargo build --bin linear_regression --features cuda
// cargo run --bin linear_regression --features cuda
