use std::{env::args, fs::read_to_string};

use parser::parse;
use runtime::execute;

mod parser;
mod runtime;

fn main() {
    let file_path = args().nth(1).expect("No path provided");
    let src = read_to_string(file_path).expect("Error reading file");
    let tokens = parse(&src);
    execute(tokens);
}
