extern crate rustacuda;
extern crate rustacuda_core;

use rustacuda::prelude::*;
use rustacuda::device::Device;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustacuda::init(CudaFlags::empty())?;

    let device_count = Device::num_devices()?;
    println!("Number of CUDA devices: {}", device_count);

    for dev in 0..device_count {
        let device = Device::get_device(dev)?;
        let name = device.name()?;
        println!("Device {}: {}", dev, name);
    }
    let device = Device::get_device(0)?;
    println!("Using device: {:?}", device);

    Ok(())
}
