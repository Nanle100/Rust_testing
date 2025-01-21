// point One: to read the values of command line arguments we pass to it, we’ll need the std::env::args function provided in Rust’s standard library.
// saving the arguement. this is when when we'd involve file_path and query

// printing with 'dbg!(args)' returns all the arguememnt our cli takes in from the terminal






// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let file_path = &args[2];

//     println!("Searching for {query}");
//     println!("In file {file_path}");

//     // from the fs library. it handles files
//     let contents = fs::read_to_string(file_path)
//         .expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
   
// }

use std::env;
use std::process;



use project_one::Config;




fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = project_one::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}








