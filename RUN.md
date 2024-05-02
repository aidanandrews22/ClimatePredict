# Group 16 Climate Predict: RUN File

## How to run the code
Because neural networks benifit drastically from GPU acceleration I have written this code to work with GPU's. This means the code can only be run if you have cuda installed on your device. 
Instructions are here: https://docs.nvidia.com/cuda/index.html#installation-guides
Can be downloaded here (for windows and linux): https://developer.nvidia.com/cuda-downloads?target_os=linux
Download for mac (untested/idk if it will work): https://developer.nvidia.com/nvidia-cuda-toolkit-developer-tools-mac-hosts

### Step One Clone The Repository:
1. open a terminal on your device
2. find a path to where you want to store the directory. The replace "/path" in step 3 with your path
3. type the command "git clone https://github.com/aidanandrews22/CS128H-Project.git ./path"

### Step Two Run Code:
1. Type the command "CD path" where 'path' is the path from step one (the one you cloned the repository into).
2. Decide which file you want to run. To run the linear_regression, polynomial_regression, or the random_forest regressions models you will need to run the command "CD Project". And to run rocket.rs or handle.rs you need to first run "CD Rocket".
3. Build the file you want to run (Optional):
    linear_regression.rs: "cargo build --bin lin --features cuda"
    polynomial_regression.rs: "cargo build --bin poly --features cuda"
    random_forest.rs: "cargo build --bin rf --features cuda"

    rocket.rs: "cargo build --bin rock"
    handle.rs: "cargo build --bin han"
4. Run the intended file:
    linear_regression.rs: "cargo run --bin lin --features cuda"
    polynomial_regression.rs: "cargo run --bin poly --features cuda"
    random_forest.rs: "cargo run --bin rf --features cuda"

    rocket.rs: "cargo run --bin rock"
    handle.rs: "cargo run --bin han"


### Notes:
If you run into any problems running the code refer to the comments at the bottom of each .rs file. Sometimes you have to run things like "cargo update"
