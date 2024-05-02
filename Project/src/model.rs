//////////////////////////////////////// dependencies ////////////////////////////////////////

// extern crate tch;
// use tch::{Device, Kind, Tensor};

//////////////////////////////////////// global variables ////////////////////////////////////////

// const DATA: &str = "./data/emission_temp_data.csv";

//////////////////////////////////////// helper functions ////////////////////////////////////////



//////////////////////////////////////// model ////////////////////////////////////////



//////////////////////////////////////// main ////////////////////////////////////////

fn main() {
//     // Check if CUDA is available and choose the device accordingly
//     let device = if tch::Cuda::is_available() {
//         println!("CUDA is available. Using GPU.");
//         Device::cuda_if_available()
//     } else {
//         println!("CUDA is not available. Using CPU.");
//         Device::Cpu
//     };
// 
//     // Define the size of the matrices
//     let size = 1000;  // Large enough to be computationally expensive
//     let iterations = 10000000;  // Number of times to perform the multiplication
// 
//     // Create two large tensors on the selected device
//     let matrix_a = Tensor::randn(&[size, size], (Kind::Float, device));
//     let matrix_b = Tensor::randn(&[size, size], (Kind::Float, device));
// 
//     // Perform matrix multiplications repeatedly
//     for i in 0..iterations {
//         let start = std::time::Instant::now();
//         let result = matrix_a.matmul(&matrix_b);
//         let duration = start.elapsed();
// 
//         println!("Iteration {}: Time taken for matrix multiplication: {:?}", i + 1, duration);
//         // Optionally print a summary of the resulting matrix to ensure computation is occurring
//         println!("Result summary: mean = {:.4}, std = {:.4}", result.mean(Kind::Float), result.std(true));
//     }
// 
     println!("Stress test completed.");
}
// 


// Command to run the code with GPU support: RUSTFLAGS="-Ctarget-cpu=native" cargo run --release --features cuda

// $env:RUSTFLAGS="-Ctarget-cpu=native"
// cargo run --release --features cuda
// cargo run --release --features cuda RUST_BACKTRACE=1

// Body Type,Sex,Diet,How Often Shower,Heating Energy Source,Transport,Vehicle Type,Social Activity,Monthly Grocery Bill,Frequency of Traveling by Air,Vehicle Monthly Distance Km,Waste Bag Size,Waste Bag Weekly Count,How Long TV PC Daily Hour,How Many New Clothes Monthly,How Long Internet Daily Hour,Energy efficiency,Recycling,Cooking_With,CarbonEmission


// cd "C:\Users\aidan\OneDrive\Documents\Code\CS128H-Project\Project"
// $Env:LIBTORCH_BYPASS_VERSION_CHECK = "1"     
// $Env:Path += ";C:\Users\aidan\libtorch\lib"  
// nvidia-smi --query-gpu=index,name,driver_version,pstate,fan.speed,temperature.gpu,utilization.gpu,utilization.memory,memory.total,memory.free,memory.used,power.draw --format=csv -l 1
