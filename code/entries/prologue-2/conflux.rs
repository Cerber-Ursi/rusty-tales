use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Crate {
    content: u8,
}

pub fn fetch<T>(_: T) -> Vec<T> {
    vec![]
}

pub fn show<T: Debug>(items: impl IntoIterator<Item = T>) {
    for (num, item) in items.into_iter().enumerate() {
        println!("Item number {}: {:?}", num, item);
    }
}