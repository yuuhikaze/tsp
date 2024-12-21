use std::io::{self, Write};
use tsp::{
    TspSolver,
    common::storage,
    mmas::{MMAS, MMASParameters},
    tsplib::TspLibInstance,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    storage::create_data_dir();
    // List TSPLIB instances
    let instances: Vec<String> = storage::read_data_dir()
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
        storage::get_intances_dir().join(instances.get(choice - 1).ok_or("Index is not in range")?);
    let mut instance = TspLibInstance::new(path);
    instance.load_data_from_file()?;
    // Compute instance solutions
    {
        let parameters = MMASParameters::new(0.98, instance.dimension(), 1.0, 2.0, 0.05, 15);
        let m = parameters.colony_size();
        let mut mmas = MMAS::new(instance, parameters);
        mmas.initialize_nn_matrix();
        // construct 100_000 solution per node
        match mmas.solve(|statistics| statistics.iteration().is_some_and(|it| it == 100_000 / m)) {
            Ok(solution) => {
                println!("Solution computed!");
                println!("Cost: {}", solution.cost);
                print!("Tour: ");
                for node in solution.visited() {
                    print!("{} → ", node);
                }
                print!("{}", solution.visited()[0]);
                println!();
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
