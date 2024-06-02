use std::fs::File;
use std::iter::Map;
use std::io::{copy, Result};
use reqwest::blocking::Client;
use scraper::{ElementRef, Html};
use scraper::html::Select;


fn extract_image_url(html: &str) -> Option<String> {
    let start_pos = html.find("src=\"")?;
    let remaining = &html[start_pos + 5..];

    // Find the end position of the URL
    let end_pos = remaining.find("\"")?;
    let url = &remaining[..end_pos];

    Some(url.to_string())
}

fn main() ->Result<()> {

    let mut koomiksite_algus = 1000;
    let koomiksite_lopp = 1020;
    while koomiksite_algus < koomiksite_lopp {
        let client = Client::new();
        let mut url = format!("https://www.giantitp.com/comics/oots{}.html" , koomiksite_algus.to_string());
        let mut res = client.get(url)
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
                let filename = format!("comics/Image{}.png", koomiksite_algus);
                let filename_clone=filename.clone();
                let mut response = client.get(&image_url)
                    .send()
                    .map_err(|err|
                        std::io::Error::new(std::io::ErrorKind::Other, err))?;
                let mut file = File::create(filename)?;

                copy(&mut response, &mut file)?;

                println!("Image downloaded successfully: {}", filename_clone);
            } else {
                println!("No items found");
            }
        }
        koomiksite_algus= koomiksite_algus + 1;
    }

    Ok(())

}
