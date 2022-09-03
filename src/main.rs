use scraper::{Html, Selector};
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt, Debug)]
/// Tool for archiving novels
struct TypedArgs {
    /// The universal resource locator of book
    book_url: String,
}

fn main() {
    let args = TypedArgs::from_args();

    let uri = Url::parse(&args.book_url).unwrap_or_else(|_| {
        println!("!!! invalid book url !!!");
        std::process::exit(1);
    });

    // let scheme = uri.scheme_str().unwrap_or("https");
    // let host = uri.host().unwrap_or_else(|| {
    //     println!("!!! invalid hostname !!!");
    //     std::process::exit(1);
    // });
    
    let table_of_contents_response = get_html_doc(args.book_url);

    let table_of_contents = Html::parse_document(&table_of_contents_response);

    let chapter_anchor_tag_selector = Selector::parse("table#chapters > tbody > tr.chapter-row > td:nth-child(1) > a").expect("Unable to parse chapter link selector");
    for anchor_tag in table_of_contents.select(&chapter_anchor_tag_selector) {
        let href = anchor_tag.value().attr("href").unwrap();

        let chapter_url = uri.join(href).expect("Invalid chapter URL").to_string();
        
        scrape_chapter(chapter_url);
    }

    // println!("{}", table_of_contents_response);
}

fn scrape_chapter(chapter_url: String) {
    let chapter_response = get_html_doc(chapter_url);
    let chapter_document = Html::parse_document(&chapter_response);

    let page_title_tag_selector = Selector::parse("title").expect("unable to parse page title tag selector");
    let title_text = chapter_document.select(&page_title_tag_selector)
        .next()
        .expect("Unable to find title tag node")
        .text()
        .next()
        .expect("Unable to find title tag");

    let chapter_header_tag_selector = Selector::parse("div.fic-header h1.font-white").expect("unable to parse chapter header tag selector");
    let chapter_header_text = chapter_document.select(&chapter_header_tag_selector)
        .next()
        .expect("Unable to find chapter header node")
        .text()
        .next()
        .expect("Unable to find chapter header");
    
    let chapter_content_selector = Selector::parse("div.chapter-inner.chapter-content").expect("unable to parse chapter content selector");
    let chapter_content_html = chapter_document.select(&chapter_content_selector)
        .next()
        .expect("Unable to find chapter content node")
        .html();

    println!("title = {}\nheader = {}\ncontent = {}", title_text, chapter_header_text, chapter_content_html);
}

/// Make request to web page and return HTML content
fn get_html_doc(url: String) -> String {
    // let user_agent = "My Rust Program 1.0";
    let client = reqwest::blocking::Client::new();

    client.get(url)
        // .header(USER_AGENT, user_agent)
        // .header(api_auth_header, api_auth_token)
        .send()
        .expect("Error making request") 
        .text()
        .expect("Invalid response")
}
