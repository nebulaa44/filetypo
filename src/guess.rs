use std::fs::File;
use std::io::Read;

#[derive(Debug, Default)]
pub struct FileType 
{
    pub mime: String,
    pub description: String,
}

impl FileType 
{
    pub fn new(mime: &str, description: &str) -> Self
    {
        Self {mime: mime.to_string(), description: description.to_string()}
    }
}

pub fn guess_type(file: &mut File) -> FileType 
{
    let mut file_bytes: Box<[u8]> = Box::new([0; 10]);
    file.read(&mut file_bytes).unwrap();

    // *.gz
    if &file_bytes[0..=1] == b"\x1F\x8B" {
        return FileType::new("application/x-gzip", "GZipped file.")
    } else {
        return FileType::default()
    }
}