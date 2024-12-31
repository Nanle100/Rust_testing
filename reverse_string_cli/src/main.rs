
// This CLI will get a string input and return it reverse

// import the env module to get the command line arguements

use std::env;

fn main() {
// cllect the command line vector
    let args: Vec<String> = env::args().collect();

    // chec if the user has enter a string
    if args.len() < 2 {
        println!("Please enter a string");
        return;
    }

    // store the string in a variable
    let input: String = args[1..].join(" ");


    // reverse the string
    let reverse: String = input.chars().rev().collect::<String>();

    //print the reversed string
    println!("The reversed string is: {reverse}");
}








