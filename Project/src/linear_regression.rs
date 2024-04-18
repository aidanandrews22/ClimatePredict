//////////////////////////////////////// dependencies ////////////////////////////////////////

use candle::{Device, Tensor};
use csv::ReaderBuilder;
use std::error::Error;

//////////////////////////////////////// global variables ////////////////////////////////////////

const DATA: &str = "./data/emission_temp_data.csv";

//////////////////////////////////////// helper functions ////////////////////////////////////////

fn process_data(data: &str) -> Result<(Tensor, Tensor), Box<dyn Error>> {
    let file = std::fs::File::open(data)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut emissions = Vec::new();
    let mut temps = Vec::new();

    for result in reader.records() {
        let record = result?;
        emissions.push(record[1].parse::<f64>()?);
        temps.push(record[2].parse::<f64>()?);
    }

    // Assuming `Device::new()` or a similar function exists to specify the device context
    let device = Device::new_cuda(0)?;

    let emissions_tensor = Tensor::from_slice(&emissions, (emissions.len(), 1), &device)?;
    let temps_tensor = Tensor::from_slice(&temps, (temps.len(), 1), &device)?;

    Ok((emissions_tensor, temps_tensor))
}

fn linear_regression(x: &Tensor, y: &Tensor) -> Result<(f64, f64), Box<dyn Error>> {
    let x_mean = x.mean(*&[0])?.to_scalar::<f64>()?;
    let y_mean = y.mean(*&[0])?.to_scalar::<f64>()?;

    let x_diff = x - x_mean;
    let y_diff = y - y_mean;

    let x_diff_tensor = x_diff?;
    let y_diff_tensor = y_diff?;

    let numerator_tensor = (&x_diff_tensor * &y_diff_tensor)?.sum(*&[])? .to_scalar::<f64>()?;
    let denominator_tensor = (&x_diff_tensor * &x_diff_tensor)?.sum(*&[])? .to_scalar::<f64>()?;

    let slope = numerator_tensor / denominator_tensor;
    let intercept = y_mean - slope * x_mean;

    Ok((slope, intercept))
}
//////////////////////////////////////// main ////////////////////////////////////////

fn main() -> Result<(), Box<dyn Error>> {
    let (emissions_tensor, temps_tensor) = process_data(DATA)?;

    match linear_regression(&emissions_tensor, &temps_tensor) {
        Ok((slope, intercept)) => {
            println!("Slope: {}", slope);
            println!("Intercept: {}", intercept);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    Ok(())
}


// cargo build --bin linear_regression
// cargo run --bin linear_regression