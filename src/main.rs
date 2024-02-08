use cpal::traits::{DeviceTrait, HostTrait};
use anyhow::Result;

fn main() -> Result<()> {
    println!("enum inputs:");

    let host = cpal::default_host();
    for input in host.input_devices()? {
        println!("- device: {:?}", input.name());
    }

    Ok(())
}
