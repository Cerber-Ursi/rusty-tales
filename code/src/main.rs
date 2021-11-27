use trybuild::TestCases;

fn main() {
    let t = TestCases::new();
    t.pass(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/entries/**/main.rs");
}
