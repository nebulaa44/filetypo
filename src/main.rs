mod guess;

use std::fs;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Args 
{
    #[arg(short, long)]
    filename: String,
}

fn main() 
{
    let args = Args::parse();
    let filename = args.filename;

    println!("Filename given: {filename}");

    let mut file = fs::File::open(filename).unwrap();
    let file_type = guess::guess_type(&mut file);

    println!("MIME Type:\t {}", file_type.mime);
    println!("Type Description: {}", file_type.description);
}
