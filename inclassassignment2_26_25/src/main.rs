use std::io::{self, Read, Write};

struct Person {
    name: String,
    sid: u32,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("Whats ur sid? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let sid = buffer.trim().parse().unwrap();

    let person = Person { name, sid };
    println!("Hi {}, your SID: {}\n", person.name, person.sid);
}
use std::fs::File;
use std::io::prelude::*;

struct Config {
    name: String,
    sid: u32,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let sid = lines.next().unwrap().parse().unwrap();

        Config { name, sid}
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("Name: {}", config.name);
    println!("SID: {}", config.sid);
}
fn main() {
    reading_from_console();
    reading_from_file();
}