use pulldown_cmark::{html, Event, Parser};
use std::{
    error::Error,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    iter::FromIterator,
    path::PathBuf,
};

pub fn convert() -> Result<(), Box<dyn Error>> {
    for lang in &["ru", "en"] {
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

        let mut output = BufWriter::new(File::create(PathBuf::from_iter(&[
            env!("CARGO_MANIFEST_DIR"),
            "..",
            "docs",
            lang,
            "index.html",
        ]))?);
        write!(
            output,
            include_str!("../html/index.html"),
            header = header,
            title = title,
            content = content
        )?;
    }
    Ok(())
}
