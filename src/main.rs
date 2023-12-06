use scraper::{Html, Selector};

const BASE_URL: &str = "https://scrapingclub.com/exercise/list_infinite_scroll/?page=";

fn main() {
    let mut page_number: u8 = 1;

    let document = reqwest::blocking::get(BASE_URL).unwrap().text().unwrap();
    let html_document = Html::parse_document(&document);

    let max_page_selector = Selector::parse(".page a").unwrap();
    let max_page = html_document
        .select(&max_page_selector)
        .map(|i| i.text().next().unwrap().to_owned().parse::<u8>())
        .filter_map(|i| i.ok())
        .max()
        .unwrap();

    while page_number <= max_page {
        let url = format!("{}{}", BASE_URL, page_number);

        let document = reqwest::blocking::get(url).unwrap().text().unwrap();
        let html_document = Html::parse_document(&document);

        let product_grid_selector = Selector::parse(".grid").unwrap();
        let product_grid = html_document.select(&product_grid_selector).next().unwrap();

        let product_selector = Selector::parse(".post").unwrap();
        let products = product_grid.select(&product_selector);

        let product_name_selector = Selector::parse("h4 a").unwrap();
        let product_price_selector = Selector::parse("h5").unwrap();
        for product in products {
            let name: &str = product // We have specify the type
                .select(&product_name_selector)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .first()
                .unwrap();

            let price: &str = product // We have specify the type
                .select(&product_price_selector)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .first()
                .unwrap();

            println!("{name} - {price}");
        }

        page_number += 1;
    }
}
