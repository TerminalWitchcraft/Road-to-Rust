extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // Change in ch13
    //let args: Vec<String> = env::args().collect();
    //let query = &args[1];
    //let filename = &args[2];
    //let config = Config::new(&args)
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Error parsing arguements: {}", err);
            process::exit(1);
        });
    println!("Searching for {} in file {}", config.query, config.filename);

    //Open the file
    //let mut f = File::open(config.filename).expect("File not found!!!");
    //let mut contents = String::new();
    //f.read_to_string(&mut contents).expect("Failed to read contents");
    //println!("The contents of the file is:\n ==========\n {}", contents);
    //run(config);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
