mod guess;

use std::fs;
use clap::Parser;

// this struct is used by clap to get cli arguments
#[derive(Parser)]
#[command(version, author, about)]
struct Args 
{
    #[arg(short, long)]
    filename: String,

    #[arg(short, long)]
    mime: bool,

    #[arg(short, long)]
    description: bool,
}

fn main() 
{
    let args = Args::parse();
    let filename = args.filename;

    let mut file = fs::File::open(filename).unwrap();
    let file_type = guess::guess_type(&mut file);

    // scriptable mode
    if args.mime { print!("{}", file_type.mime) }
    else if args.description { print!("{}", file_type.description) }
    
    // if scriptable mode isn't enabled fallback to friendly ui
    else
    {
        println!("MIME Type:\t  {}", file_type.mime);
        println!("Type Description: {}", file_type.description);
    }
}