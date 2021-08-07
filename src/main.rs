mod vm;
mod error;
mod scanner;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};

fn main() {
    // Pass cmd args to executable file by cargo: `cargo run -- [your flags]`
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        panic!("To many arguments!\n");
    }
}

fn run_repl() {
    loop {
        let mut contents = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut contents).unwrap();
        interprete(&contents);
    }
}

fn run_file(path: &String) {
    let mut file = File::open(path).expect("No such file!\n");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    interprete(&contents);
}

fn interprete(source: &String) {
    println!("{}", source);
}

