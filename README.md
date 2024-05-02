# CS 128 Honors Project Proposal

## Group Name: ClimatePredict

### Members
- Saket Reddy (saketr3)
- Aidan Andrews (aidansa2)
- Ben Hug (benhug2)

## Project Introduction
Our project will be a regression model that predicts what the global temperature will be given a certain amount of global carbon emissions. The user can input different amounts of carbon emissions and see what Earth’s global temperature would be if the world emitted that much CO2. The goal of this project is to raise awareness about how exactly carbon emissions can affect Earth’s temperature.

We chose this project because all of us are interested in machine learning so we knew we wanted to create a machine learning model. Additionally, we wanted our project to have a real-world impact and we think this project accomplishes this because it raises awareness about climate change.

## Technical Overview
The core of ClimatePredict will be a regression model designed to forecast global temperature changes based on varying levels of carbon dioxide emissions. Here’s the planned technical breakdown and roadmap for our project:

- **Data Collection & Preprocessing:**
  - Collect historical data on global CO2 emissions and temperature changes.
  - Clean and preprocess the data to ensure it’s suitable for machine learning training including normalization and handling missing values.

- **Neural Network Design:**
  - Design a machine learning architecture suitable for regression analysis likely consisting of multiple layers including input, hidden, and output layers.
  - The input layer will accept CO2 emissions data, and the output layer will predict the corresponding global temperature.
  - Thinking of using an LSTM neural network

- **Development Environment Setup:**
  - Set up a Rust on VSCODE with necessary libraries and frameworks such as Candle for machine learning training and implementation.

- **Implementation and Training:**
  - Implement the model using Rust.
  - Train the model with the prepared datasets adjusting hyperparameters as necessary to improve accuracy and reduce overfitting.

- **Testing and Evaluation:**
  - Test the model with unseen data to evaluate its predictive accuracy.    
  - Refine the model based on test results optimizing for better performance.    

- **User Interface Development:**
  - A mock user interface is already done. It is sufficient for testing but may be improved for aesthetic.
  - Develop a simple user interface that allows users to input CO2 emission levels and receive predicted global temperature changes.
  - Ensure the interface is user-friendly and accessible potentially using a web framework compatible with Rust.

- **Documentation and Deployment:**
  - Document the project extensively including the model design, data sources, and user instructions.
  - Prepare the project for deployment ensuring all dependencies are properly managed and the application is stable.

## Checkpoints Plan
- By Checkpoint 1 (4/3 to 4/7):
  - Complete data collection and preprocessing.
  - Have a basic architecture designed and initial training started.

- By Checkpoint 2 (4/17 to 4/21):
  - Complete the training and initial testing.
  - Start developing the user interface.

- By Final Submission (5/1):
  - Finalize the model and user interface.
  - Complete testing and evaluation.
  - Ensure comprehensive documentation and prepare for project presentation.

## Possible Challenges
- Finding a way to make the input layer of the model with input data from datasets.
- Learning and researching the new syntax needed to compose correct code.
- Gathering accurate and useful data from different datasets.
- Comparing the yearly temperature data with emissions data in a manner that allows us to come to a sound prediction.

## References
- Inspiration Behind Neural Networks in Rust:
  - [Candle project examples](https://github.com/huggingface/candle/tree/main/candle-examples/examples)
  - [Neural network tutorial with candle (just look under section called “Training simple dense neural networks with Candle”)](https://rust.marcoinacio.com/data/candle/#training-simple-dense-neural-networks-with-candle)
  - [Simple neural network tutorial (voting)](https://medium.com/@igumnovnsk/simplified-rust-example-of-training-a-neural-network-based-on-the-candle-framework-by-hugging-face-cf1ccd85a936)
- Datasets Used in the Project:
  - See Datasets.md
- Other Potential Datasets:
  - [Carbon footprint data](https://www.kaggle.com/datasets/dumanmesut/individual-carbon-footprint-calculation?resource=download)
  - [Explore the climate indicators](https://www.epa.gov/climate-indicators/dig-data)
  - [Annual surface temperature change](https://climatedata.imf.org/datasets/4063314923d74187be9596f10d034914/explore)
  - [CO₂ and Greenhouse Gas Emissions - Our World in Data](https://ourworldindata.org/co2-and-greenhouse-gas-emissions)
  - [Other climate datasets](https://ourworldindata.org/search?q=CO2)
