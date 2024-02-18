mod connectivity;
mod hotspot;
mod platforms;

use std::env::args;
use crate::connectivity::{Connectivity, WifiConnectionError};
use crate::platforms::{Config, WiFi};

use std::process::{Command, Stdio};

fn main() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    match wifi.connect("CSIS_MH", "") {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successfull."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }

    let speed = wifi.speed().unwrap();
    println!("Speed is: {}", speed);

    Ok(())
}
