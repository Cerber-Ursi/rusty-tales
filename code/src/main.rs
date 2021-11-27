use batch_run::Batch;

fn main() {
    let list: Vec<&str> = include!(concat!(env!("OUT_DIR"), "/list.rs"));
    println!("{:?}", list);

    let t = Batch::new();
    t.run_pass("entries/**/main.rs");
    t.run().unwrap();
}
