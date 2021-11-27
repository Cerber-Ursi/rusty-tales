use std::error::Error;

mod chapter;
mod consts;
mod index;
mod processing;

struct Chapter {
    id: String,
    title: String,
    brief: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    for lang in &["ru", "en"] {
        let chapters: Vec<Chapter> = chapter::list()?
            .map(|name| chapter::convert(lang, &name))
            .collect::<Result<_, _>>()?;
        index::convert(lang, chapters)?;
    }
    Ok(())
}
