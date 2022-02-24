use clap::Parser;
use http::uri;
use hyper::Client;
use hyper_tls::HttpsConnector;
use kuchiki::traits::TendrilSink;

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
    let uri: uri::Uri = args.book_url.parse()?;

    // Await the response...
    let resp = client.get(uri.clone()).await?;

    let body_as_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let html = String::from_utf8(body_as_bytes.to_vec())?;

    let document = kuchiki::parse_html().one(html);

    let title_css_selector = "div.fic-header h1.font-white";
    let book_title_match = document.select_first(title_css_selector).unwrap();
    let book_title_node = book_title_match.as_node();
    let book_title_text_node = book_title_node.first_child().unwrap();
    let book_title = book_title_text_node.as_text().unwrap().borrow();

    println!("{}", book_title);
    
    let chapter_link_css_selector = "table#chapters > tbody > tr.chapter-row > td:nth-child(1) > a";
    for css_match in document.select(chapter_link_css_selector).unwrap() {
        let attrs = css_match.attributes.borrow();
        let href = attrs.get("href").unwrap();

        let u = uri::Builder::new()
            .scheme(uri.scheme_str().unwrap())
            .authority(uri.host().unwrap())
            .path_and_query(href)
            .build()
            .unwrap();

        println!("{}", u);
    }

    Ok(())
}
