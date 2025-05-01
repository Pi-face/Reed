#[derive(Debug)] 
pub enum Command { 
    Load {filename: String, variable : String},
    Filter { variable : String, condition: String },
    Selection { variable : String, fields: Vec<String> },
    Save { variable : String, output: String },
}
