use std::env;
use std::process;

use lic_rs::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{} cores", num_cpus::get());
    println!("{} cpus", num_cpus::get_physical());

    lic_rs::run(config);
}
