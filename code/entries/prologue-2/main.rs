use stderrlog;

mod conflux;
mod earth;

fn main() {
    stderrlog::new().verbosity(4).init().expect("Failed to initialize logger");
    let mut items = conflux::CRATES.clone();
    earth::deliver_to(&mut items);
    conflux::show(items);
}
