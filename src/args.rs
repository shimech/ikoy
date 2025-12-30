use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the JavaScript file
    #[arg(name = "script.js")]
    pub file: Option<String>,

    /// Evaluate JavaScript code and print result
    #[arg(short, long, value_name = "...")]
    pub print: Option<String>,
}

impl Args {
    pub fn script(&self) -> String {
        if let Some(file) = &self.file {
            fs::read_to_string(file).unwrap()
        } else if let Some(print) = &self.print {
            print.clone()
        } else {
            panic!(
                "No JavaScript code provided. Please provide JavaScript file or inline script with --print option."
            );
        }
    }
}
