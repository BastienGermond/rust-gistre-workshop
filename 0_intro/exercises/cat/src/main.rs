use std::env;
use std::io::Read;

pub fn cat(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", cat(&args[1]));
}
