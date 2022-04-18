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

macro_rules! execute_inner {
    ($env:expr, $name:ident, $is_post: ident) => {
        if let Some(p) = $env {
            let mut f = std::fs::File::open(p).expect("Can not open ZOLA_TEMPLATE_POST env path!");
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)
                .expect("Can not open ZOLA_TEMPLATE_POST env path!");
            execute(Some(&buf), &$name, $is_post).expect("Failed to create File!");
        } else {
            execute(None, &$name, $is_post).expect("Failed to create File!");
        }
    };
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
            execute_inner!(post_template, post_name, true);
        }
        ZolaGenCommand::NewPage(NewPageCommand { post_name }) => {
            execute_inner!(page_template, post_name, false);
        }
    }
}
