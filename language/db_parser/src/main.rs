const FILE_PATH: &str = "../set.mm";

use db_parser::terminals::symbol;
use std::fs::File;
use std::io::Read;
use nom::Parser;

#[cfg(debug_assertions)]
const STACK_SIZE: usize = 8 * 1024 * 1024;

fn main() {
    #[cfg(debug_assertions)]
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(main_thread)
        .unwrap()
        .join()
        .unwrap();
    #[cfg(not(debug_assertions))]
    println!("not debug assertions");
    main_thread();
}

fn main_thread() {
    let mut file = File::open(FILE_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let tree = db_parser::document::document(&contents.as_bytes()).unwrap().1;
}