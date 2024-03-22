// This will be the file with the entire model for the project
// The model that i have built so far is a skeleton and will not work until updated

use candle::nn::{Module, Sequential, Linear};
use candle::optim::SGD;
use candle::tensor::Variable;

fn main() {
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
