use batch_run::Batch;

fn main() {
    let list: Vec<&str> = include!(concat!(env!("OUT_DIR"), "/list.rs"));
    let t = Batch::new();
    for item in list {
        // A glob pattern here is a hack to force batch_run ignore non-existent paths
        t.run_pass(format!("entries/{}/main*.rs", item));
    }
    t.run().unwrap();
}
