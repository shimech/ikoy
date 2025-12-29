use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Evaluate JavaScript and print result
    #[arg(short, long)]
    pub print: String,
}
