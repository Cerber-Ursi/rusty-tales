use pulldown_cmark::{html, Parser};
use std::{
    fs::{read_to_string, File},
    io::BufWriter,
    path::PathBuf,
};

fn main() {
    let input: PathBuf = [env!("CARGO_MANIFEST_DIR"), "markdown", "index.md"]
        .into_iter()
        .collect();
    let output: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "docs", "index.html"]
        .into_iter()
        .collect();

    html::write_html(
        BufWriter::new(File::create(output).expect("Could not open output file")),
        Parser::new(&read_to_string(input).expect("Could not read input file")),
    )
    .expect("Could not write HTML");
}
