use batch_run::Batch;

fn main() {
    let t = Batch::new();
    t.run_pass("entries/**/main.rs");
    t.run().unwrap();
}
