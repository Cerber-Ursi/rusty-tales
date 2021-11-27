use std::fmt::Debug;
use lazy_static::lazy_static;
use log::{info, trace};

lazy_static! {
    pub static ref CRATES: Vec<Crate> = {
        trace!("Creating lazy_static");
        vec![Crate {content: 42}]
    };
}

#[derive(Default, Debug, Clone)]
pub struct Crate {
    content: u8,
}

pub fn show<T: Debug>(items: impl IntoIterator<Item = T>) {
    for (num, item) in items.into_iter().enumerate() {
        info!("Item number {}: {:?}", num, item);
    }
}