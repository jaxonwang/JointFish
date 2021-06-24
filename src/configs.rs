use std::path::PathBuf;

#[derive(Debug)]
pub struct Config{
    pub world_size: usize,
    pub here_id: usize,
    pub bcf_files: Vec<PathBuf>
}
