// A simple tool for generating meaningless project names
use std::env;
use rand::{self, Rng};

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    let mut num: i32 = 1; // number of names to generate (default = 1)

    // Check whether any command line arguments were given
    if args.len() > 1{
        let num_str = &args[1]; // This should be the desired number of names

        // Try to convert the argument to the type of num
        num = match num_str.parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                eprintln!("error: second argument not an integer");
                help();
                return;
            },
        };
    
    println!("Generating {} names", num);

    }

    // Import the wordlists as strings
    let adjectives = include_str!("adj");
    let nouns = include_str!("noun");

    // Split wordlist strings into vectors, each containing one word
    let adjectives: Vec<String> = adjectives.split_whitespace().map(str::to_string).collect();
    let nouns: Vec<String> = nouns.split_whitespace().map(str::to_string).collect();

    // Find the length of each vector and use it to select a random noun and adjective
    let num_adjectives = adjectives.len();
    let num_nouns = nouns.len();

    for _ in 0..num{
        let adjective_index = rand::thread_rng().gen_range(0, num_adjectives);
        let adjective = &adjectives[adjective_index];

        let noun_index = rand::thread_rng().gen_range(0, num_nouns);
        let noun = &nouns[noun_index];
    
        // println!("{} adjectives and {} nouns", num_adjectives, num_nouns);
        // println!("Picked noun number {} and noun number {}", adjective_index, noun_index);

        // Concatenate the selected adjective and noun into a suggested name and print it
        let suggested_name = [adjective, " ", noun].concat();
        println!("{}", suggested_name);
    }
}

fn help() {
    println!("Usage: prophetic_tongue [NUMBER; default = 1]
    Generate NUMBER random names.");
}