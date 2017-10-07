use std::io::BufReader;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in args {
        let path = Path::new(&arg);
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file
        };

        let reader = BufReader::new(file);        
        for l in reader.lines() {
            println!("{}", l.unwrap());
        }

    }
}