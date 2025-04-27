use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // collects all the arguments in the console, then collects the string list into a Vector or Array.
    if args.len() < 2 {
        eprintln!("Usage: reed <scriptfile>");
        std::process::exit(1);
        // if the user did not provide more than 1 it didnt contain the file in the list.
    }
    let filename = &args[1];
    println!("Filename: {}", filename);
}

 