use reqwest::blocking::Client;
use scraper::{Html, Selector};



fn main() {
    println!("Hello, world!");

    let client = Client::new();
    let mut res = client.get("https://www.giantitp.com/comics/oots1278.html")
                                .send()
                                .unwrap();
    let body = res.text().unwrap();
    let _document = Html::parse_document(&body);
    let pildiLink = scraper::Selector::parse("td").unwrap();

    let image_selector = _document.select(&pildiLink).map(|x|x.inner_html());

    let PuhasPildiLink = image_selector.zip(1..12)
        .for_each((|(item,num)| println!("{}. {}",num, item)));



   /* for element in pildiLink.(&pildiLink) {
        let link_selector = Selector::parse("a").unwrap();

       for link in element.select(link_selector){
           if let Some(href) = link.value().attr("href") {
               println!("{}", href);
           }
       }
    }*/

}
