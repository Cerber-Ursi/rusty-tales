use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let vars = env::vars()
        .filter(|(name, _)| name.starts_with("CARGO_FEATURE"))
        .map(|s| s.0.split_at(14).1.to_lowercase().replace("_", "-"));
    let mut out = File::create(env::var("OUT_DIR").unwrap() + "/list.rs").unwrap();
    write!(out, "vec![").unwrap();
    for var in vars {
        write!(out, "{:?},", var).unwrap();
    }
    write!(out, "]").unwrap();
}
