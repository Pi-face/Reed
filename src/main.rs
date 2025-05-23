mod command;
mod parser;

use command::Command;
use parser::parse_line;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // collects all the arguments in the console, then collects the string list into a Vector or Array.
    if args.len() < 2 {
        eprintln!("Usage: reed <scriptfile>");
        std::process::exit(1);
        // if the user did not provide more than 1 it didnt contain the file in the list.
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for(index, line) in contents.lines().enumerate(){
        match parse_line(line) {
            Some(command) => println!("Line {}: {:?}", index + 1, command),
            None => println!("Line {}: Invalid command", index + 1),
        }   
    }
    let test = Command::Filter {
        variable: "variable".to_string(),
        condition: "condition".to_string(),
    };
    println!("{:?}", test);
}
