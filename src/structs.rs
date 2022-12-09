use std::{
    fs::{self, create_dir_all, read_to_string, write},
    path::Path,
    process,
};

use chrono::{DateTime, Utc};
use scraper::Selector;
use serde::{Deserialize, Serialize};

use crate::{date_to_gg_string, home_dir, parse_gg_string_for_date};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub latest_page: String,
    pub cache_dir: String,
}

#[derive(Clone)]
pub struct ConfigFile {
    pub path: String,
}
impl ConfigFile {
    pub fn read(&self) -> Config {
        // make sure path exists (if not, yell at user)
        if !Path::new(&self.path).exists() {
            // TODO: make it show an error in the GUI with this data, instead of crashing
            if &self.path == &format!("{}/.config/ggg/config.json", home_dir()) {
                let dir_created = create_dir_all(format!("{}/.config/ggg", home_dir()));
                if let Err(why) = dir_created {
                    eprintln!("Couldn't create the config directory: {:?}", why);
                    process::exit(1);
                };

                let wrote = write(
                    &self.path,
                    serde_json::to_string(&Config {
                        latest_page: "20021104".to_string(),
                        cache_dir: format!("{}/.cache/ggg", home_dir()),
                    })
                    .expect("Couldn't serialize a new config file"),
                );

                if let Err(why) = wrote {
                    eprintln!("Couldn't write to the empty config file: {}", why);
                    process::exit(1);
                }
            } else {
                eprintln!("There's no config file at that location.");
                process::exit(1);
            }
        }
        if !Path::new(&self.path).is_file() {
            // TODO: make it show an error in the GUI with this data, instead of crashing
            eprintln!("That config file exists, but it's not a file.");
            process::exit(1);
        }

        // make sure it's got valid config data in it (if not, check if it's empty and create it (if not empty, yell at user))
        let file = read_to_string(self.clone().path).expect("Couldn't read config file");
        if file.is_empty() {
            let wrote = write(
                &self.path,
                serde_json::to_string(&Config {
                    latest_page: "20021104".to_string(),
                    cache_dir: format!("{}/.cache/ggg", home_dir()),
                })
                .expect("Couldn't serialize a new config file"),
            );

            if let Err(why) = wrote {
                eprintln!("Couldn't write to the empty config file: {}", why);
                process::exit(1);
            }
        };

        let deserialized: Result<Config, serde_json::Error> = serde_json::from_str(&file[..]);

        if let Err(why) = deserialized {
            // TODO: make it show an error in the GUI with this data, instead of crashing
            eprintln!("Couldn't parse the config file: {:?}", why);
            process::exit(1);
        } else {
            deserialized.expect("literally how")
        }
    }

    pub fn update_latest_page(self, date: String) {
        let mut read = self.read();
        read.latest_page = date;
        let string: String = serde_json::to_string(&read).expect("Couldn't serialize the config");
        write(self.path, string).expect("Couldn't write to the config file");
    }
}

#[derive(Debug)]
pub struct GirlGeniusPage {
    pub current_url: String,
    pub current_image: String,
    pub previous_url: Option<String>,
    pub next_url: Option<String>,
}
impl GirlGeniusPage {
    pub async fn new(date: DateTime<Utc>) -> GirlGeniusPage {
        let url = date_to_gg_string(date);
        let result_option = reqwest::get(url).await;

        let result = match result_option {
            Ok(r) => r,
            Err(why) => {
                eprintln!("Couldn't fetch a GG page's HTML: {:?}", why);
                process::exit(1);
            }
        };

        let text = match result.text().await {
            Ok(t) => t,
            Err(why) => {
                eprintln!("Couldn't get the text of a GG page: {:?}", why);
                process::exit(1);
            }
        };

        // println!("{}", text);

        let html = scraper::Html::parse_document(&text[..]);
        let selector = scraper::Selector::parse("div[id=\"topnav\"] > a").unwrap();

        let topnav_data = html
            .select(&selector)
            .map(|c| GirlGeniusTopnavData {
                id: c.value().attr("id").unwrap().to_string(),
                href: c.value().attr("href").unwrap().to_string(),
            })
            .collect::<Vec<GirlGeniusTopnavData>>();

        let image_selector = Selector::parse("img[alt=\"Comic\"]").unwrap();
        let image_url_vec = html
            .select(&image_selector)
            .map(|c| c.value().attr("src").unwrap().to_string())
            .collect::<Vec<String>>();

        let image_url = &image_url_vec[0];

        // println!("{}", image_url);
        let next_url = if topnav_data.iter().find(|d| d.id == "topnext").is_some() {
            Some(
                topnav_data
                    .iter()
                    .find(|d| d.id == "topnext")
                    .expect("literally how")
                    .href
                    .clone(),
            )
        } else {
            None
        };

        let previous_url = if topnav_data.iter().find(|d| d.id == "topprev").is_some() {
            Some(
                topnav_data
                    .iter()
                    .find(|d| d.id == "topprev")
                    .expect("literally how")
                    .href
                    .clone(),
            )
        } else {
            None
        };

        GirlGeniusPage {
            current_url: date_to_gg_string(date),
            current_image: image_url.clone().to_string(),
            next_url,
            previous_url,
        }
    }

    pub async fn save(&self, cache_dir: &String) -> String {
        let filename = format!(
            "{}.jpg",
            parse_gg_string_for_date(self.current_url.clone())
                .format("%Y%m%d")
                .to_string()
        );
        let filepath = format!("{}/{}", cache_dir, filename);
        if !self.check(cache_dir) {
            println!("check is false, downloading image");
            let image = match reqwest::get(self.current_image.clone()).await {
                Ok(image) => match image.bytes().await {
                    Ok(bytes) => bytes,
                    Err(why) => {
                        eprintln!(
                            "Couldn't get the bytes of the current page's image: {:?}",
                            why
                        );
                        process::exit(1);
                    }
                },
                Err(why) => {
                    eprintln!("Couldn't get the current image: {:?}", why);
                    process::exit(1);
                }
            };

            // println!("{}", yansi::Color::Magenta.paint(filename.clone()));

            // println!("{}", yansi::Color::Magenta.paint(filepath.clone()));
            let wrote = fs::write(filepath.clone(), image);
            if let Err(why) = wrote {
                eprintln!(
                    "Couldn't save the image to `{}/{}`: {:?}",
                    cache_dir, filename, why
                );
                process::exit(1);
            }
        };

        filepath.clone()
    }
    pub fn check(&self, cache_dir: &String) -> bool {
        let filename = format!(
            "{}.jpg",
            parse_gg_string_for_date(self.current_url.clone())
                .format("%Y%m%d")
                .to_string()
        );

        let contents = fs::read_dir(cache_dir)
            .expect("ok please just don't mess with the cache folder when the program is running");
        let collected_contents: Vec<String> = contents
            .into_iter()
            .map(|f| f.unwrap().file_name().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        // false

        let has = collected_contents.contains(&filename);

        println!("does the cache dir have the image? {:?}", has);
        has
    }
}

#[derive(Debug)]
struct GirlGeniusTopnavData {
    id: String,
    href: String,
}

