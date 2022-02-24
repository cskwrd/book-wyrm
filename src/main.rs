use clap::Parser;
use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

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
    let mut resp = client.get(uri).await?;

    // Write HTML asynchronously to stdout...
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
