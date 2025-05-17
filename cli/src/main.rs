
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    jar: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.jar);
}
