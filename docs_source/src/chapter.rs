use crate::{consts, processing::highlight_rust, Chapter};
use pulldown_cmark::{html, CowStr, Event, Parser};
use std::{
    error::Error,
    fs::{create_dir_all, read_to_string, File},
    io::{BufRead, BufReader, BufWriter, Write},
    iter::FromIterator,
    path::PathBuf,
    rc::Rc,
    sync::Mutex,
};

#[derive(Clone)]
struct ChapterParser<'a>(Rc<Mutex<Parser<'a>>>);
impl<'a> Iterator for ChapterParser<'a> {
    type Item = Event<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self
            .0
            .lock()
            .expect("Panic inside the ChapterParser")
            .next()
        {
            Some(Event::Html(CowStr::Borrowed("<cut>"))) => None,
            x => x,
        }
    }
}

pub fn list() -> Result<impl Iterator<Item = String>, Box<dyn Error>> {
    Ok(BufReader::new(File::open(PathBuf::from_iter(&[
        env!("CARGO_MANIFEST_DIR"),
        "markdown",
        "chapters.lst",
    ]))?)
    .lines()
    .map(|line| line.expect("Error reading chapters list")))
}

pub(crate) fn convert(lang: &str, name: &str) -> Result<Chapter, Box<dyn Error>> {
    let mut content = String::new();
    let mut brief = String::new();
    let mut code = String::new();
    let input = PathBuf::from_iter(&[
        env!("CARGO_MANIFEST_DIR"),
        "markdown",
        lang,
        name,
        "story.md",
    ]);

    let md = read_to_string(input.clone())?;
    let mut events = ChapterParser(Rc::new(Mutex::new(Parser::new(&md))));
    let title = match events.nth(1) {
        Some(Event::Text(text)) => text.to_string(),
        x => Err(format!(
            "Malformed Markdown file {} - no title, got {:?} instead",
            input.display(),
            x
        ))?,
    };
    let _ = events.next();
    html::push_html(&mut brief, events.clone());
    html::push_html(
        &mut content,
        (&*events.0.lock().expect("Panic in the first iteration")).clone(),
    );
    html::push_html(
        &mut code,
        highlight_rust(Parser::new(&read_to_string(
            input.with_file_name("code.md"),
        )?))
        .into_iter(),
    );

    let mut out_path = PathBuf::from_iter(&[env!("CARGO_MANIFEST_DIR"), "..", "docs", lang, name]);
    create_dir_all(&out_path)?;
    out_path.push("chapter.html");
    let mut output = BufWriter::new(File::create(&out_path)?);
    write!(
        output,
        include_str!("../html/chapter.html"),
        title = title,
        content = content,
        to_code = consts::TO_CODE[lang],
    )?;

    out_path.set_file_name("code.html");
    let mut output = BufWriter::new(File::create(out_path)?);
    write!(
        output,
        include_str!("../html/code.html"),
        title = title,
        content = code,
        to_chapter = consts::TO_CHAPTER[lang],
    )?;

    Ok(Chapter {
        title,
        brief,
        id: name.to_owned(),
    })
}
