pub mod structs;
pub mod ui;

use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;
use std::{env, process};
use structs::{ConfigFile, GirlGeniusPage, GggCacheData};

pub fn home_dir() -> String {
    match std::env::consts::OS {
        // WINDOWS: THE BEST FUCKING OPERATING SYSTEM IN EXISTENCE
        "windows" => env::var("USERPROFILE").expect("Couldn't get the $USERPROFILE env var"),
        _ => env::var("HOME").expect("Couldn't get the $HOME env var"),
    }
}

pub fn parse_gg_string_for_date(string: String) -> DateTime<Utc> {
    let regex = Regex::new(r"(\d{4})(\d{2})(\d{2})").expect("Couldn't compile regex");

    let captures_maybe = regex.captures(&string[..]);

    if let None = captures_maybe {
        eprintln!("Couldn't parse date info from a GG date string");
        process::exit(1);
    };

    let captures = regex.captures(&string[..]).expect("literally how");

    Utc.ymd(
        captures[1].to_string().parse::<i32>().unwrap(),
        captures[2].to_string().parse::<u32>().unwrap(),
        captures[3].to_string().parse::<u32>().unwrap(),
    )
    .and_hms(12, 0, 0)
}

pub fn date_to_gg_string(date: DateTime<Utc>) -> String {
    format!(
        "https://www.girlgeniusonline.com/comic.php?date={}",
        date.format("%Y%m%d").to_string()
    )
}

pub async fn next(conf: ConfigFile, cache_dir: &String) -> Option<(GirlGeniusPage, String)> {
    let current_page = GirlGeniusPage::new(parse_gg_string_for_date(conf.read().latest_page)).await;
    if let Some(next) = current_page.next_url {
        // println!("{}", next);
        let next_page = GirlGeniusPage::new(parse_gg_string_for_date(next)).await;
        let next_page_date = parse_gg_string_for_date(next_page.current_url.clone());
        conf.update_latest_page(next_page_date.format("%Y%m%d").to_string());

        let filepath = next_page.save(cache_dir).await;

        Some((next_page, filepath))
    } else {
        None
    }
}

pub async fn previous(conf: ConfigFile, cache_dir: &String) -> Option<(GirlGeniusPage, String)> {
    if conf.read().latest_page == String::from("20021104") {
        None
    } else {
        let current_page =
            GirlGeniusPage::new(parse_gg_string_for_date(cp)).await;
        if let Some(prev) = current_page.previous_url {
            // println!("{}", next);
            let prev_page = GirlGeniusPage::new(parse_gg_string_for_date(prev)).await;
            let prev_page_date = parse_gg_string_for_date(prev_page.current_url.clone());
            conf.update_latest_page(prev_page_date.format("%Y%m%d").to_string());

            let filepath = prev_page.save(cache_dir).await;

            Some((prev_page, filepath))
        } else {
            None
        }
    }
}
