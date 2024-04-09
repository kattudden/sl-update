use scraper::{Html, Selector};
use regex::Regex;
use std::process::exit;


fn main() {
    let standins_url = "https://www.bscyb.ch/tabelle-sl";

    let response = reqwest::blocking::get(standins_url).unwrap().text().unwrap();

    let document = Html::parse_document(&response);

    let row_selector = Selector::parse("tr").unwrap();

    let ranks = document.select(&row_selector).map(|x| x.html());

    let re_pattern = match Regex::new(r"<td[^>]*>([^<]+)") {
        Ok(re_pattern) => re_pattern,
        Err(error) => {
            eprintln!("Failed to compile pattern. Error {error}");
            exit(1);
        }
    };

    for rank in ranks {

        let mut row = vec![];

        for cap in re_pattern.captures_iter(&rank) {
            row.push(cap.get(1).unwrap().as_str());
        }

        println!("row: {:?}", row);
    }

}