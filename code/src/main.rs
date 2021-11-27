use batch_run::Batch;

fn main() {
    let t = Batch::new();
    t.run_pass(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/entries/**/main.rs");
    t.run();
}
