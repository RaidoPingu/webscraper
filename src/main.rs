use std::iter::Map;
use std::mem::take;
use reqwest::blocking::Client;
use scraper::{ElementRef, Html, Selector};
use regex::Regex;
use scraper::html::Select;
use scraper::Node::Element;
use tokio::select;


fn extract_image_url(html: &str) -> Option<String> {
    // Find the start position of the URL
    let start_pos = html.find("src=\"")?;
    let remaining = &html[start_pos + 5..]; // Skip "src=\""

    // Find the end position of the URL
    let end_pos = remaining.find("\"")?;
    let url = &remaining[..end_pos];

    Some(url.to_string())
}

fn main() {
    let client = Client::new();
    let mut res = client.get("https://www.giantitp.com/comics/oots1278.html")
        .send()
        .unwrap();
    let body = res.text().unwrap();
    let _document = Html::parse_document(&body);
    let pildiLink = scraper::Selector::parse("td").unwrap();

    let image_selector: Map<Select, fn(ElementRef) -> String> = _document.select(&pildiLink).map(|x| x.inner_html());

    let items: Vec<String> = image_selector.collect();

    if let Some(last_item) = items.get(10) {
        println!("last item: {:?}", last_item);
        if let Some(image_url) = extract_image_url(last_item) {
            println!("Extracted url: {}", image_url);

        } else {
            println!("No items found");
        }


        //let re = Regex::new(r"^https?://[a-z0-9-.]{2,}\.[a-z]{2,4}(:[0-9]{2,5})?/?.*$").unwrap();


        //let PuhasPildiLink = image_selector.zip(1..12)
        //   .for_each((|(item,num)| println!("{}. {}",num, item)));


        //let mut WebAdress =vec![];
        //for (_, [path]) in re.captures_iter(PuhasPildiLink).map(|c|c.extract()) {
        //    res
        // }
        // println!("{:?}", WebAdress);

        /* for element in pildiLink.(&pildiLink) {
        let link_selector = Selector::parse("a").unwrap();

       for link in element.select(link_selector){
           if let Some(href) = link.value().attr("href") {
               println!("{}", href);
           }
       }
    }*/
    }
}
