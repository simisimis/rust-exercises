use clap::Parser;
use howcool::Args;

fn main() {
    let args = Args::parse();
    args.print();
    println!("{:?}", howcool::response(&args));
}
