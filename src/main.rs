use clap::Parser;
use hyper::Client;
use hyper_tls::HttpsConnector;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The universal resource locator of book
    book_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Parse an `http::Uri`...
    let uri = args.book_url.parse()?;

    // Await the response...
    let resp = client.get(uri).await?;

    println!("Response status: {}", resp.status());

    Ok(())
}
