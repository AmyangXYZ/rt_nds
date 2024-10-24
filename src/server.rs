use crate::cache::Cache;

pub struct Server {
    pub cache: Cache,
}

impl Server {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
        }
    }
}
