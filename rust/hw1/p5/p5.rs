use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("missing file path!");
        return;
    }

    let data: String = fs::read_to_string(&args[1]).expect("unable to read file!");
    println!("{}", data);
}