//////////////////////////////////////// dependencies ////////////////////////////////////////

use candle::{DType, Device, Tensor};
use candle_nn::{
    linear,
    loss::mse,
    optim::{AdamW, Optimizer, ParamsAdamW},
    Linear, Module, VarBuilder, VarMap,
};

use csv::ReaderBuilder;
use std::{error::Error}; // simd::LaneCount
use tqdm::tqdm;

use plotpy::{Contour, Plot, Surface};
use plotters::prelude::*;

//////////////////////////////////////// global variables ////////////////////////////////////////

const DATA: &str = "./data/emission_temp_data.csv";

//////////////////////////////////////// helper functions ////////////////////////////////////////

fn process_data(data: &str, device: &Device) -> Result<(Tensor, Tensor, Tensor), Box<dyn Error>> {
    let file = std::fs::File::open(data).expect("Unable to open file process_data()");
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut years = Vec::new();
    let mut emmisions = Vec::new();
    let mut temps = Vec::new();

    for result in reader.records() {
        let record = result?;
        years.push(record[0].parse::<f64>().expect("Unable to parse years process_data()"));
        emmisions.push(record[1].parse::<f64>().expect("Unable to parse emmisions process_data()"));
        temps.push(record[2].parse::<f64>().expect("Unable to parse temps process_data()"));
    }

    let years_tensor = Tensor::from_slice(&years, (years.len(),), device);
    let emissions_tensor = Tensor::from_slice(&emmisions, (emmisions.len(),), device);
    let temps_tensor = Tensor::from_slice(&temps, (temps.len(),), device);

    let years_tensor = years_tensor?;
    let emissions_tensor = emissions_tensor?;
    let temps_tensor = temps_tensor?;
    
    let data_tensor = Tensor::stack(&[&years_tensor, &emissions_tensor, &temps_tensor], 1)?;

    println!("Tensor shape: {:?}", data_tensor.shape());
    println!("Tensor data: \n{}", data_tensor);

    Ok((years_tensor, emissions_tensor, temps_tensor))
}



//////////////////////////////////////// model ////////////////////////////////////////

struct ClimatePredict { // This is the struct representing the neural network model
    ln1: Linear,
    ln2: Linear,
    ln3: Linear,
}

impl ClimatePredict { // This is the constructor for the ClimatePredict struct
    fn new(vs: VarBuilder) -> Result<Self, Box<dyn Error>> {
        let ln1 = linear(1, 64, vs.pp("ln1"))?; // first hidden layer with 64 neurons
        let ln2 = linear(64, 32, vs.pp("ln2"))?; // second hidden layer with 32 neurons
        let ln3 = linear(32, 1, vs.pp("ln3"))?; // output neuron
        Ok(Self { ln1, ln2, ln3 })
    }
}

impl Module for ClimatePredict {
    fn forward(&self, xs: &Tensor) -> candle::Result<Tensor> { // this method defines the forward pass of the neural network
        let xs = self.ln1.forward(xs)?; // pass tensor to first layer
        let xs = xs.relu()?; // apply relu activation function
        let xs = self.ln2.forward(&xs)?; // pass to second layer
        let xs = xs.relu()?; // apply relu
        self.ln3.forward(&xs) // pass to output layer
    }
}


//////////////////////////////////////// main ////////////////////////////////////////

pub fn main() -> Result<(), Box<dyn Error>> { // main function sets up the device, variable map, optimizer, and training loop
    let device = Device::cuda_if_available(0)?;
    println!("Using device: {:?}", device);

    
    
    let data_processed = process_data(DATA, &device);
    let (years_tensor, emissions_tensor, temps_tensor) = process_data(DATA, &device)?;
    
    let varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let climate_predict = ClimatePredict::new(vs)?;
    
    Ok(())
}



// Command to run the code with GPU support: RUSTFLAGS="-Ctarget-cpu=native" cargo run --release --features cuda

// $env:RUSTFLAGS="-Ctarget-cpu=native"
// cargo run --release --features cuda
// cargo run --release --features cuda RUST_BACKTRACE=1

// Body Type,Sex,Diet,How Often Shower,Heating Energy Source,Transport,Vehicle Type,Social Activity,Monthly Grocery Bill,Frequency of Traveling by Air,Vehicle Monthly Distance Km,Waste Bag Size,Waste Bag Weekly Count,How Long TV PC Daily Hour,How Many New Clothes Monthly,How Long Internet Daily Hour,Energy efficiency,Recycling,Cooking_With,CarbonEmission
