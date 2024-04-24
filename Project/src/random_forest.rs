use smartcore::ensemble::random_forest_regressor::{RandomForestRegressor, RandomForestRegressorParameters};
use smartcore::model_selection::train_test_split;
use smartcore::metrics::mean_squared_error;
use smartcore::linalg::basic::matrix::DenseMatrix;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Body Type")]
    body_type: String,
    #[serde(rename = "Sex")]
    sex: String,
    #[serde(rename = "Diet")]
    diet: String,
    #[serde(rename = "How Often Shower")]
    how_often_shower: String,
    #[serde(rename = "Heating Energy Source")]
    heating_energy_source: String,
    #[serde(rename = "Transport")]
    transport: String,
    #[serde(rename = "Social Activity")]
    social_activity: String,
    #[serde(rename = "Monthly Grocery Bill")]
    monthly_grocery_bill: f64,
    #[serde(rename = "Frequency of Traveling by Air")]
    frequency_of_traveling_by_air: String,
    #[serde(rename = "Vehicle Monthly Distance Km")]
    vehicle_monthly_distance_km: f64,
    #[serde(rename = "Waste Bag Weekly Count")]
    waste_bag_weekly_count: i32,
    #[serde(rename = "How Long TV PC Daily Hour")]
    how_long_tv_pc_daily_hour: i32,
    #[serde(rename = "How Many New Clothes Monthly")]
    how_many_new_clothes_monthly: i32,
    #[serde(rename = "How Long Internet Daily Hour")]
    how_long_internet_daily_hour: i32,
    #[serde(rename = "Energy efficiency")]
    energy_efficiency: String,
    #[serde(rename = "CarbonEmission")]
    carbon_emission: f64,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data from a CSV file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)  // Ensure headers are expected
        .from_path(".\\data\\Carbon Emission.csv")?;
    
    let mut dataset: Vec<Record> = Vec::new();
    
    for result in rdr.deserialize() {
        let record: Record = result?;
        dataset.push(record);
    }
    
    // Optionally, print some records to check if they are loaded correctly
    for record in dataset.iter().take(5) {
        println!("{:?}", record);
    }

    // Assuming 'dataset' is now a Vec of numeric features including the target variable
    let (x, y): (Vec<Vec<f64>>, Vec<f64>) = dataset.iter().map(|r| (vec![/* feature vector */], r.carbon_emission)).unzip();

    // Convert data into a Matrix for SmartCore
    let x = DenseMatrix::from_2d_vec(&x);

    // Split the dataset into training and testing datasets
    let (x_train, x_test, y_train, y_test) = train_test_split(&x, &y, 0.2, true, Some(42)); // added seed for reproducibility

    // Define the model with default parameters
    let rf_params = RandomForestRegressorParameters::default();
    let rf = RandomForestRegressor::fit(&x_train, &y_train, rf_params)?;

    // Evaluate the model
    let preds = rf.predict(&x_test)?;
    let mse = mean_squared_error(&y_test, &preds);
    let rmse = mse.sqrt(); // Calculate RMSE for better interpretation

    println!("Mean Squared Error: {}", mse);
    println!("Root Mean Squared Error: {}", rmse);

    Ok(())
}


// cargo build --bin rf --features cuda
// cargo run --bin rf --features cuda