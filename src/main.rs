#![allow(dead_code)]
use std::io::{self, Write};

use tsplib::TspLibInstance;

mod mmas;
mod tsplib;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::create_data_dir();
    // List TSPLIB instances
    let instances: Vec<String> = utils::read_data_dir()
        .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
        .collect();
    if instances.is_empty() {
        return Err(
            "$XDG_DATA_HOME/tsp/instances was scanned and no instances were found, add some first"
                .into(),
        );
    }
    // Select an instance
    for (i, instance) in instances.iter().enumerate() {
        println!("{}. {}", i + 1, instance);
    }
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let choice: usize = input.trim().parse()?;
    // Load selected TSPLIB instance
    let path =
        utils::get_intances_dir().join(instances.get(choice - 1).ok_or("Index is not in range")?);
    let mut tsp_lib_instance = TspLibInstance::new(path);
    tsp_lib_instance.load()?;
    println!("{:#?}", tsp_lib_instance);
    // Compute instance solution using MMAS
    Ok(())
}
