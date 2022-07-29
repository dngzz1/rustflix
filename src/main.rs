use clap:: {Parser};
use args::RustflixArgs;

mod args;

fn main() {
    let args = RustflixArgs::parse();
    println!("{:?}", args);
}
