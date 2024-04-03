//////////////////////////////////////// dependencies ////////////////////////////////////////
use candle::nn::{Module, Sequential, Linear};
use candle::optim::SGD;
use candle::tensor::Variable;


//////////////////////////////////////// global variables ////////////////////////////////////////

let path_carbon = "./data/carbon-emissions.csv";
let path_temp = "./data/global-temperature.csv";
let data = "./emission_temp_data.csv"


//////////////////////////////////////// helper functions ////////////////////////////////////////

fn ProcessData() {
}


//////////////////////////////////////// model ////////////////////////////////////////

fn model() {
    let mut model = Sequential::new(vec![
        Box::new(Linear::new(10, 5)),
        Box::new(Linear::new(5, 1)),
    ]);

    let mut optimizer = SGD::new(model.parameters(), 0.01);

    for epoch in 0..100 {
        let input = Variable::randn([1, 10]); // FAKE INPUT DATA
        let target = Variable::randn([1, 1]); // FAKE TARGET PREDICTION

        let output = model.forward(&input);

        let loss = (output - target).pow(2).mean();

        optimizer.zero_grad();
        loss.backward();

        optimizer.step();
        
        if epoch % 10 == 0 {
            println!("Epoch {}: Loss = {}", epoch, loss.data()[0]);
        }
    }
}
