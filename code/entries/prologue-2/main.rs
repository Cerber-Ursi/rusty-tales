mod conflux;
mod earth;

fn main() {
    let cur_crate = conflux::Crate::default();
    let mut items = conflux::fetch(cur_crate);
    earth::deliver_to(&mut items);
    conflux::show(items);
}
