use clap::Parser;
use futures::executor;
use http::uri;
use hyper::Client;
use hyper_tls::HttpsConnector;
use scraper::{Html, Selector};

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

    let uri: uri::Uri = args.book_url.parse()?;
    
    let scheme = uri.scheme_str().unwrap_or("https");
    let host = uri.host().unwrap_or_else(|| {
        println!("!!! invalid hostname !!!");
        std::process::exit(1);
    });
    
    let book_response = client.get(uri.clone()).await?;

    let body_as_bytes = hyper::body::to_bytes(book_response.into_body()).await?;
    let book_html = String::from_utf8(body_as_bytes.to_vec())?;
    let document = Html::parse_document(book_html.as_str());

    let chapter_anchor_tag_selector = Selector::parse("table#chapters > tbody > tr.chapter-row > td:nth-child(1) > a").unwrap();
    for anchor_tag in document.select(&chapter_anchor_tag_selector) {
        let href = anchor_tag.value().attr("href").unwrap();

        let chapter_uri = uri::Builder::new()
            .scheme(scheme)
            .authority(host)
            .path_and_query(href)
            .build()
            .unwrap();

        let chapter_client = client.clone();
        let chapter_html_future = async {
            let chapter_response = chapter_client.get(chapter_uri).await.unwrap();

            let chapter_as_bytes = hyper::body::to_bytes(chapter_response.into_body()).await.unwrap();
            String::from_utf8(chapter_as_bytes.to_vec()).unwrap()
        };
        let chapter_html = executor::block_on(chapter_html_future);
        let chapter_document = Html::parse_document(chapter_html.as_str());

        let page_title_tag_selector = Selector::parse("title").unwrap();
        let title_text = chapter_document.select(&page_title_tag_selector).next().unwrap().text().next().unwrap();
        
        let chapter_content_selector = Selector::parse("div.chapter-inner.chapter-content").unwrap();
        let chapter_content_html = chapter_document.select(&chapter_content_selector).next().unwrap().html();

        println!("title = {}\ncontent = {}", title_text, chapter_content_html);
    }

    Ok(())
}
