use std::fs::File;
use std::iter::Map;
use std::io::{copy, Result};
use reqwest::blocking::Client;
use scraper::{ElementRef, Html};
use scraper::html::Select;


fn extract_image_url(html: &str) -> Option<String> {
    // Find the start position of the URL
    let start_pos = html.find("src=\"")?;
    let remaining = &html[start_pos + 5..]; // Skip "src=\""

    // Find the end position of the URL
    let end_pos = remaining.find("\"")?;
    let url = &remaining[..end_pos];

    Some(url.to_string())
}

fn main() ->Result<()> {
    let client = Client::new();
    let mut res = client.get("https://www.giantitp.com/comics/oots1278.html")
        .send()
        .map_err(|err|
            std::io::Error::new(std::io::ErrorKind::Other, err))?;
    let body = res.text().unwrap();
    let _document = Html::parse_document(&body);
    let pildi_link = scraper::Selector::parse("td").unwrap();

    let image_selector: Map<Select, fn(ElementRef) -> String> = _document.select(&pildi_link).map(|x| x.inner_html());

    let items: Vec<String> = image_selector.collect();

    if let Some(last_item) = items.get(10) {
        println!("last item: {:?}", last_item);
        if let Some(image_url) = extract_image_url(last_item) {
            println!("Extracted url: {}", image_url);
            let filename = "comics/Image.png";
            let mut response = client.get(&image_url)
                .send()
                .map_err(|err|
                    std::io::Error::new(std::io::ErrorKind::Other, err))?;
            let mut file = File::create(filename)?;

            copy(&mut response, &mut file)?;

            println!("Image downloaded successfully: {}", filename);
        } else {
            println!("No items found");
        }
    }

    Ok(())

}
