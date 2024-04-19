// Main code that will call different files (ie. model.rs and any ui files)
mod model;
mod test;
pub mod linear_regression;

fn main() {
    let _ = model::main();
    // let _ = test::main();
}
