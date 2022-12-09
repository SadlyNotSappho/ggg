use clap::{arg, ArgAction, Command};
use ggg::{
    home_dir,
    structs::ConfigFile,
    ui::GggUi
    // previous
};
use iced::{Settings, Application};
use std::{fs, path::Path, process};

fn cli() -> Command {
    Command::new("ggg")
        .name("Girl Genius GUI")
        .about("A GUI program to read the webcomic \"Girl Genius\"")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(false)
        .arg(
            arg!(-c --config_file "The config file to use. Defaults to ~/.config/ggg/config.json")
                .action(ArgAction::Set),
        )
        .arg(
            arg!(-d --cache_dir "The cache directory to use. Defaults to ~/.cache/ggg")
                .action(ArgAction::Set),
        )
        .arg(arg!(-s --string "string").action(ArgAction::Set))
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    let other_cfgpath = format!("{}/.config/ggg/config.json", home_dir());
    let config_file_path = matches
        .get_one::<String>("config_file")
        .unwrap_or(&other_cfgpath);

    let config = ConfigFile {
        path: config_file_path.clone(),
    }
    .read();
    let cache_dir_path = matches
        .get_one::<String>("cache_dir")
        .unwrap_or(&config.cache_dir);

    std::env::set_var("ggg_config_path", config_file_path.clone());
    std::env::set_var("ggg_cache_path", cache_dir_path);

    check_cache_dir(cache_dir_path);

    // previous(ConfigFile{path:std::env::var("ggg_config_path").unwrap()}, &std::env::var("ggg_cache_path").unwrap()).await;

    GggUi::run(Settings::default()).unwrap();

    // process::exit(1);

    // let filepath = GirlGeniusPage::new(parse_gg_string_for_date(config.latest_page))
    //     .await
    //     .save(cache_dir_path)
    //     .await;
    // println!("show file {}", filepath);
    // process::Command::new("firefox")
    //     .arg(filepath)
    //     .output()
    //     .expect("fuck");

    // println!("ran firefox command");

    // loop {
    //     print!("Input action: ");
    //     let action: String = text_io::read!("{}\n");
    //     let action = action.trim();

    //     match action {
    //         "next" => {
    //             let next_res = next(
    //                 ConfigFile {
    //                     path: config_file_path.clone(),
    //                 },
    //                 cache_dir_path,
    //             )
    //             .await;
    //             match next_res {
    //                 Some((_page, filepath)) => {
    //                     println!("show file {}", filepath);
    //                     process::Command::new("firefox")
    //                         .arg(filepath)
    //                         .output()
    //                         .expect("fuck");
    //                 }
    //                 None => {
    //                     eprintln!("Either something went terribly wrong, or this is the last page.")
    //                 }
    //             }
    //         }
    //         "prev" => {
    //             let prev_res = previous(
    //                 ConfigFile {
    //                     path: config_file_path.clone(),
    //                 },
    //                 cache_dir_path,
    //             )
    //             .await;
    //             match prev_res {
    //                 Some((_page, filepath)) => {
    //                     println!("show file {}", filepath);
    //                     process::Command::new("firefox")
    //                         .arg(filepath)
    //                         .output()
    //                         .expect("fuck");
    //                 }
    //                 None => {
    //                     eprintln!("Either something went terribly wrong, or this is the last page.")
    //                 }
    //             }
    //         }
    //         "previous" => {
    //             let prev_res = previous(
    //                 ConfigFile {
    //                     path: config_file_path.clone(),
    //                 },
    //                 cache_dir_path,
    //             )
    //             .await;
    //             match prev_res {
    //                 Some((_page, filepath)) => {
    //                     println!("show file {}", filepath);
    //                     process::Command::new("firefox")
    //                         .arg(filepath)
    //                         .output()
    //                         .expect("fuck");
    //                 }
    //                 None => {
    //                     eprintln!("Either something went terribly wrong, or this is the last page.")
    //                 }
    //             }
    //         }
    //         _ => println!("Unknown action: {}", action),
    //     }
    // }
}

fn check_cache_dir(cache_dir: &String) {
    let dir = Path::new(cache_dir);
    if dir.exists() {
        if dir.is_file() {
            eprintln!("The cache directory exists, but it's a file.");
            process::exit(1);
        }
    } else {
        if dir.to_str().unwrap() != format!("{}/.cache/ggg", home_dir()) {
            eprintln!("Please create that cache directory.");
            process::exit(1);
        } else {
            let created = fs::create_dir_all(dir);
            if let Err(why) = created {
                eprintln!("Couldn't create the cache directory: {:?}", why);
                process::exit(1);
            }
        }
    }
}

