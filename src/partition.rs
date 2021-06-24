use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub fn bcf_owner(file_name: &Path, world_size: usize) -> usize {
    let mut h = DefaultHasher::new();
    file_name.as_os_str().hash(&mut h);
    h.finish() as usize % world_size
}
