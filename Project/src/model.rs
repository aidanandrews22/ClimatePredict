//////////////////////////////////////// dependencies ////////////////////////////////////////
use candle::{DType, Device, Tensor};
use candle_nn::{
    linear,
    loss::mse,
    optim::{AdamW, Optimizer, ParamsAdamW},
    Linear, Module, VarBuilder, VarMap,
};
use std::error::Error;
use tqdm::tqdm;


//////////////////////////////////////// global variables ////////////////////////////////////////

let path_carbon = "./data/carbon-emissions.csv";
let path_temp = "./data/global-temperature.csv";
let data = "./emission_temp_data.csv"


//////////////////////////////////////// helper functions ////////////////////////////////////////

fn ProcessData(data_path: &str, device: &Device) -> Result<(Tensor, Tensor), Box<dyn Error>> {
    let file = File::open(data_path)?;
    let reader = BufReader::new(file);

    let mut emissions = Vec::new();
    let mut temperatures = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let values: Vec<f32> = line
            .split(',')
            .skip(1)
            .map(|x| x.parse().unwrap_or(0.0))
            .collect();
        
        if values.len() == 2 {
            emissions.push(values[0]);
            temperatures.push(values[1]);
        }
    }

    let emissions_tensor = Tensor::from_slice(&emissions)?.view((emissions.len(), 1)).to_device(device)?;
    let temperatures_tensor = Tensor::from_slice(&temperatures)?.view((temperatures.len(), 1)).to_device(device)?;

    Ok((emissions_tensor, temperatures_tensor))
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

fn main() -> Result<(), Box<dyn Error>> { // main function sets up the device, variable map, optimizer, and training loop
    let device = Device::cuda_if_available(0)?;
    println!("Using device: {:?}", device);
    let varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let climate_predict = ClimatePredict::new(vs)?;
    let mut optimizer =
        AdamW::new(varmap.all_vars(), ParamsAdamW::default())?;

    // TODO: Load the dataset and preprocess it into xs_train (CO2 emissions) and ys_train (global temperature)
    let (emissions_tensor, temperatures_tensor) = ProcessData(data, &device)?;

    // Split the data into training and validation sets
    let train_ratio = 0.8;
    let train_size = (emissions_tensor.size()[0] as f64 * train_ratio) as i64;

    // training
    let xs_train = emissions_tensor.i(..train_size);
    let ys_train = temperatures_tensor.i(..train_size);

    // validation
    let xs_val = emissions_tensor.i(train_size..);
    let ys_val = temperatures_tensor.i(train_size..);

    let n_epochs = 10_000;
    let mut losses_val = Vec::<f32>::with_capacity(n_epochs);

    for epoch in tqdm(0..n_epochs) {
        if epoch % (n_epochs / 10) == 0 || epoch == n_epochs - 1 {
            // TODO: Compute validation loss on a validation set
            losses_val
                .push(mse(&dnn.forward(&xs_val)?, &ys_val)?.to_scalar()?);
        }

        let gradients =
            mse(&climate_predict.forward(&xs_train)?, &ys_train)?.backward()?;
        optimizer.step(&gradients)?;
    }
    println!("Losses on validation set: {:?}", losses_val);
    Ok(())
}

// Command to run the code with GPU support: RUSTFLAGS="-Ctarget-cpu=native" cargo run --release --features cuda