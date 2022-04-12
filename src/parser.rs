use std::{io::Write, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::{format_description, OffsetDateTime};
use toml::Value;

#[derive(Debug, Deserialize, Serialize)]
struct NewItem {
    title: String,
    date: Option<String>,
    #[serde(flatten)]
    other: Option<Value>,
}

const TEMPLATE_POST_DEFAULT: &[u8] = include_bytes!("../template_post_default.toml");
const TEMPLATE_PAGE_DEFAULT: &[u8] = include_bytes!("../template_page_default.toml");

fn read_template(s: &[u8]) -> Result<NewItem> {
    Ok(toml::from_slice(s)?)
}

fn apply_templete(title: &str, new_item: &mut NewItem) -> Result<()> {
    new_item.title = title.to_string();
    if new_item.date.is_some() {
        let t = OffsetDateTime::now_utc();
        let format = format_description::well_known::Rfc3339;
        let date = t.format(&format)?;
        new_item.date = Some(date);
    }

    Ok(())
}

fn write_to_file(title: &str, new_item: &mut NewItem, is_post: bool) -> Result<()> {
    let s = format!("+++\n{}+++\n", toml::to_string(new_item)?);

    if is_post {
        let path = Path::new(".").join("content").join(format!("{}.md", title));
        let mut f = std::fs::File::create(&path)?;
        f.write_all(s.as_bytes())?;
        println!("File: {} is create!", path.display());
    } else {
        let mut path = Path::new(".").join(title);
        std::fs::create_dir_all(&path)?;
        path.push("_index.md");
        let mut f = std::fs::File::create(&path)?;
        f.write_all(s.as_bytes())?;
        println!("File: {} is create!", path.display());
    }

    Ok(())
}

pub fn execute(buf: Option<&[u8]>, title: &str, is_post: bool) -> Result<()> {
    let mut template = if let Some(buf) = buf {
        read_template(buf)?
    } else if is_post {
        read_template(TEMPLATE_POST_DEFAULT)?
    } else {
        read_template(TEMPLATE_PAGE_DEFAULT)?
    };
    apply_templete(title, &mut template)?;
    write_to_file(title, &mut template, is_post)?;

    Ok(())
}

#[test]
fn test() {
    let mut new_item = NewItem {
        title: "".to_string(),
        date: Some("".to_string()),
        other: None,
    };
    apply_templete("qaq", &mut new_item).unwrap();
    dbg!(toml::to_string(&new_item).unwrap());
}
