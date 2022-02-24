use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The universal resource locator of book
    book_url: String,
}

fn main() {
    let args = Args::parse();

    println!("URL: '{}'", args.book_url);
}
