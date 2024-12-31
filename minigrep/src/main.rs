// About this cli - grep:
// What grep Does:
// ---Search for Patterns: It looks for specific sequences of characters (patterns) in files or input streams.
// --- Display Matching Lines: It outputs the lines from the input where the specified pattern is found.
// --- Filter Text: You can use it to process text files or streams by filtering only the relevant information you need. 

// How grep Works:
// --- Input: grep takes a pattern (which can be simple text or a more complex regular expression) and a file or stream to search.
// --- Pattern Matching: It compares the pattern to each line of text in the input.
// --- Output: It prints the lines that match the pattern, optionally highlighting the match.


// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     dbg!(args);
// }



// Now we need to save the values of the two arguments in variables so we can use the values throughout the rest of the program.
use std::env;
use std::fs;

fn main() {
    // accepting commandline arguement
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
// Let’s run this program again with the arguments test and sample.txt (this will be my command: cargo run -- test sample.txt)
// the first arguememnt is my reference which will be test
// and the second arguement is my file/reference path which will be sample.txt

