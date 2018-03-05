#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate serde_derive;

pub mod storage;

pub use storage::{Storage, StorageBuilder};
