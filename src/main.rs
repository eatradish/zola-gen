use std::{io::Read, path::Path};

use clap::{Parser, Subcommand};
use parser::execute;

mod parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    subcommand: ZolaGenCommand,
}

#[derive(Subcommand, Debug)]
enum ZolaGenCommand {
    /// Create a new post
    NewPost(NewPostCommand),
    /// Create a new page
    NewPage(NewPageCommand),
}

#[derive(Parser, Debug)]
struct NewPostCommand {
    #[clap()]
    post_name: String,
}

#[derive(Parser, Debug)]
struct NewPageCommand {
    #[clap()]
    post_name: String,
}


fn main() {
    let args = Args::parse();
    let post_template = std::env::var("ZOLA_TEMPLATE_POST").ok();
    let page_template = std::env::var("ZOLA_TEMPLATE_PAGE").ok();
    if !Path::new("./content").is_dir() || !Path::new("./config.toml").is_file() {
        eprintln!("Please open your zola blog source directory!");
        std::process::exit(1);
    }
    match args.subcommand {
        ZolaGenCommand::NewPost(NewPostCommand { post_name }) => {
            if let Some(p) = post_template {
                let mut f = std::fs::File::open(p).expect("Can not open ZOLA_TEMPLATE_POST env path!");
                let mut buf = Vec::new();
                f.read_to_end(&mut buf).expect("Can not open ZOLA_TEMPLATE_POST env path!");
                execute(Some(&buf), &post_name, true).expect("Failed to create File!");
            } else {
                execute(None, &post_name, true).expect("Failed to create File!");
            }
        }
        ZolaGenCommand::NewPage(NewPageCommand { post_name }) => {
            if let Some(p) = page_template {
                let mut f = std::fs::File::open(p).expect("Can not open ZOLA_TEMPLATE_POST env path!");
                let mut buf = Vec::new();
                f.read_to_end(&mut buf).expect("Can not open ZOLA_TEMPLATE_POST env path!");
                execute(Some(&buf), &post_name, false).expect("Failed to create File!");
            }  else {
                execute(None, &post_name, false).expect("Failed to create File!");
            }
        }
    }
}