use scraper::{Html, Selector};
use regex::Regex;
use std::process::exit;



#[derive(Debug)]
struct Team {
    pos: i8, // rank
    name: String,
    games_played: i8,
    victories: i8, 
    draws: i8,
    losses: i8,
    gf: i8, // goals for
    ga: i8, // goals against
    pm: i8, // +/-
    points: i8
}

fn get_team(html_string: String) -> Result<Team, String> {

    // define Regex Pattern to match the values in the table rows.
    let re_pattern = match Regex::new(r"<td[^>]*>([^<]+)") {
        Ok(re_pattern) => re_pattern,
        Err(error) => {
            eprintln!("Failed to compile pattern. Error {error}");
            exit(1);
        }
    };

    // define empty list to collect values.
    let mut row = vec![];

    // loop throu heystack...
    for cap in re_pattern.captures_iter(&html_string) {
        row.push(cap.get(1).unwrap().as_str());
    }

    if row.len() == 0 {
        return Err("Empty row found.".to_string());
    }

    let team = Team { 
        pos: row[0].parse::<i8>().unwrap_or(0),
        name: row[1].to_string(),
        games_played: row[2].parse::<i8>().unwrap_or(0),
        victories: row[3].parse::<i8>().unwrap_or(0),
        draws: row[4].parse::<i8>().unwrap_or(0),
        losses: row[5].parse::<i8>().unwrap_or(0),
        gf: row[6].parse::<i8>().unwrap_or(0),
        ga: row[7].parse::<i8>().unwrap_or(0),
        pm: row[8].parse::<i8>().unwrap_or(0),
        points: row[9].parse::<i8>().unwrap_or(0)
    };

    return Ok(team);

}


fn main() {
    let standins_url = "https://www.bscyb.ch/tabelle-sl";

    let response = reqwest::blocking::get(standins_url).unwrap().text().unwrap();

    let document = Html::parse_document(&response);

    let row_selector = Selector::parse("tr").unwrap();

    let ranks = document.select(&row_selector).map(|x| x.html());

    let mut ranking: Vec<Team> = Vec::new();

    for rank in ranks {
        let team = match get_team(rank) {
            Ok(team) => team,
            Err(_) => {
                continue;
            }
        };
        ranking.push(team);
    }

    println!("Ranking: {:?}", ranking);


}