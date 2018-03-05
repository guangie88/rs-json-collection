extern crate chrono;
#[macro_use]
extern crate derive_getters;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Getters)]
pub struct Storage {
    path: String,
    capacity: u64,
    used: u64,
    remaining: u64,
    used_prop: f64,
    remaining_prop: f64,
    datetime: DateTime<Local>,
}

#[derive(Default)]
pub struct StorageBuilder {
    pub path: String,
    pub capacity: u64,
    pub used: u64,
}

impl StorageBuilder {
    pub fn path<P: Into<String>>(mut self, path: P) -> StorageBuilder {
        self.path = path.into();
        self
    }

    pub fn capacity(mut self, capacity: u64) -> StorageBuilder {
        self.capacity = capacity;
        self
    }

    pub fn used(mut self, used: u64) -> StorageBuilder {
        self.used = used;
        self
    }

    pub fn build(&self) -> Storage {
        Storage {
            path: self.path.clone(),
            capacity: self.capacity,
            used: self.used,
            remaining: self.capacity - self.used,
            used_prop: self.used as f64 / self.capacity as f64,
            remaining_prop: (self.capacity - self.used) as f64
                / self.capacity as f64,
            datetime: Local::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_api() {
        const CAPACITY: u64 = 12345;
        const USED: u64 = 1234;

        let mut builder = StorageBuilder::default();

        let v = builder
            .path("/")
            .capacity(CAPACITY)
            .used(USED)
            .build();

        // assigned fields
        assert_eq!("/", v.path());
        assert_eq!(CAPACITY, *v.capacity());
        assert_eq!(USED, *v.used());

        // inferred fields
        assert_eq!(CAPACITY - USED, *v.remaining());
        assert_eq!(USED as f64 / CAPACITY as f64, *v.used_prop());
        assert_eq!(
            (CAPACITY - USED) as f64 / CAPACITY as f64,
            *v.remaining_prop()
        );

        // datetime is automatic
    }
}
