use crate::{consts, Chapter};
use itertools::Itertools;
use pulldown_cmark::{html, Event, Parser};
use std::{
    error::Error,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    iter::FromIterator,
    path::PathBuf,
};

pub(crate) fn convert(lang: &str, chapters: Vec<Chapter>) -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    let input = PathBuf::from_iter(&[env!("CARGO_MANIFEST_DIR"), "markdown", lang, "index.md"]);

    let md = &read_to_string(input.clone())?;
    let mut events = Parser::new(&md);
    let title = match events.nth(1) {
        Some(Event::Text(text)) => text.to_string(),
        x => Err(format!(
            "Malformed Markdown file {} - no title, got {:?} instead",
            input.display(),
            x
        ))?,
    };
    let header = match events.nth(2) {
        Some(Event::Text(text)) => text.to_string(),
        x => Err(format!(
            "Malformed Markdown file {} - no header, got {:?} instead",
            input.display(),
            x
        ))?,
    };
    html::push_html(&mut content, events.skip(1));

    let mut out_path =
        PathBuf::from_iter(&[env!("CARGO_MANIFEST_DIR"), "..", "docs", lang, "index.html"]);
    for (index, chunk) in chapters.chunks(5).enumerate() {
        if index > 0 {
            out_path.set_file_name("index-".to_string() + &index.to_string() + ".html");
        }
        let mut output = BufWriter::new(File::create(&out_path)?);
        let chapters = chunk
            .iter()
            .map(|Chapter { id, title, brief }| {
                format!(
                    "
                    <h3><a href=\"{id}/chapter.html\">{title}</a></h3>
                    <p>{brief}</p>
                    <a href=\"{id}/code.html\">{to_code}</a>
                    ",
                    id = id,
                    title = title,
                    brief = brief,
                    to_code = consts::TO_CODE[lang]
                )
            })
            .join("<hr>");
        write!(
            output,
            include_str!("../html/index.html"),
            header = header,
            title = title,
            content = content,
            chapters = chapters,
        )?;
    }
    Ok(())
}
