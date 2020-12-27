// A simple tool for generating meaningless project names
use rand::{self, Rng};

fn main() {
    // Import the wordlists as strings
    let adjectives = include_str!("adj");
    let nouns = include_str!("noun");

    // Split wordlist strings into vectors, each containing one word
    let adjectives: Vec<String> = adjectives.split_whitespace().map(str::to_string).collect();
    let nouns: Vec<String> = nouns.split_whitespace().map(str::to_string).collect();

    // Find the length of each vector and use it to select a random noun and adjective
    let num_adjectives = adjectives.len();
    let adjective_index = rand::thread_rng().gen_range(0, num_adjectives);
    let adjective = &adjectives[adjective_index];

    let num_nouns = nouns.len();
    let noun_index = rand::thread_rng().gen_range(0, num_nouns);
    let noun = &nouns[noun_index];
    
    // println!("{} adjectives and {} nouns", num_adjectives, num_nouns);
    // println!("Picked noun number {} and noun number {}", adjective_index, noun_index);

    // Concatenate the selected adjective and noun into a suggested name and print it
    let suggested_name = [adjective, " ", noun].concat();
    println!("{}", suggested_name);
}
