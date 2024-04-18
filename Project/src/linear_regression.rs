//////////////////////////////////////// dependencies ////////////////////////////////////////

use candle::{Device, Tensor};
use csv::ReaderBuilder;
use std::error::Error;
use std::io;

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
    // Calculate the mean of the emissions tensor: ∀x ∈ emissions_tensor: (Σx_i + x_i+1) / #of elements
    let x_mean = x.mean(*&[0])?.to_scalar::<f64>()?;
    // Calculate the mean of the temperature tensor: ∀x ∈ temps_tensor: (Σx_i + x_i+1) / #of elements
    let y_mean = y.mean(*&[0])?.to_scalar::<f64>()?;

    // calculates the difference between each element and the mean, for computing the covariance/variance
    let x_diff = x - x_mean;
    let y_diff = y - y_mean;

    let x_diff_tensor = x_diff?;
    let y_diff_tensor = y_diff?;

    // calculates the sum of the products of differences. This is the covariance between x and y (how much the variables change each other)
    let numerator_tensor = (&x_diff_tensor * &y_diff_tensor)?.sum(*&[])? .to_scalar::<f64>()?;
    // calculates the sum of the squared residuals of x. This is the variance (how spread out the data is)
    let denominator_tensor = (&x_diff_tensor * &x_diff_tensor)?.sum(*&[])? .to_scalar::<f64>()?;

    // This is the ratio of the covariance to variance β=cov(x,y)/var(x)
    let slope = numerator_tensor / denominator_tensor;
    // This is the y intercept of the regression line α = y(mean) - βx(mean)
    let intercept = y_mean - slope * x_mean;

    Ok((slope, intercept))
}

//////////////////////////////////////// main ////////////////////////////////////////

fn main() -> Result<(), Box<dyn Error>> {
    let (emissions_tensor, temps_tensor) = process_data(DATA)?;

    let (slope, intercept) = linear_regression(&emissions_tensor, &temps_tensor)?;

    loop {
        println!("Enter emission value (g CO2/kWh) or type 'exit' to quit:");
        let mut emission_input = String::new();
        io::stdin().read_line(&mut emission_input)?;
        if emission_input.trim().eq("exit") {
            break;
        }

        let emission_value: f64 = match emission_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Calculate temperature change based on input predicted_temp = β * input + α (slope intercept formula)
        let predicted_temp = slope * emission_value + intercept;
        println!("Predicted temperature change: {:.2} °C", predicted_temp);
    }

    Ok(())
}



// cargo build --bin linear_regression
// cargo run --bin linear_regression
