// src/utils/ocl_utils.rs
#[allow(unused)]
use ocl::{ProQue, Buffer, Kernel, Context, Device, Platform, Queue, Error};


/// Initializes OpenCL and returns the platform, device, context, and command queue.
pub fn setup_opencl() -> Result<(Platform, Device, Context, Queue), Error> {
    // Step 1: Get the first available platform
    let platform = Platform::list().into_iter().next().ok_or_else(|| Error::from("Platform not found"))?;
    println!("Platform: {}", platform.name()?);

    // Get the first available device
    let device = Device::list_all(platform)?
        .into_iter()
        .next()
        .ok_or_else(|| Error::from("Device not found"))?;
    println!("Device: {}", device.name()?);

    // Create a context for the device
    let context = Context::builder()
        .platform(platform)
        .devices(device.clone())
        .build()?;

    // Create a command queue
    let queue = Queue::new(&context, device.clone(), None)?;

    println!("OpenCL device and context initialized successfully!");
    Ok((platform, device, context, queue))
}
