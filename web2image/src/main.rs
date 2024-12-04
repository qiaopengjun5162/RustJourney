// use clap::{AppSettings, Clap};

// #[derive(Debug, Clap)]
// #[clap(version = "0.1", author = "Xiao Qiao <marcus@marcus-larsen.dk>")]
// #[clap(setting = AppSettings::ColoredHelp)]
// struct Opts {
//     /// output file
//     #[clap(short, long, default_value = "/tmp/snapshot.jpg")]
//     output: String,
//     /// url to cpature
//     url: String,
// }

// fn main() {
//     let opts: Opts = Opts::parse();
//     println!("{:#?}", opts);
// }
mod web2image;
use std::{ffi::OsStr, path::Path};

use clap::Parser;
use url::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// output file
    #[clap(short, long, default_value = "/tmp/snapshot.jpg")]
    output: String,
    /// url to cpature
    url: String,
}

fn get_file_ext(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "jpg" | "png" | "jpeg" => Some(ext),
                _ => None,
            }
        })
}

fn valid_url(url: &str) -> Result<(), String> {
    match Url::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err("You must provide a valid url.".into()),
    }
}
fn main() {
    let opts: Opts = Opts::parse();

    println!("{:#?}", opts);

    let format = get_image_format(Path::new(&opts.output));

    // web2image(opts.url, opts.output);
}
