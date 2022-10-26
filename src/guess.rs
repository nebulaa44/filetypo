use std::fs::File;
use std::io::Read;

#[derive(Debug)]
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

    // tar.gz
    if file_bytes[0] == 31 && file_bytes[1] == 139 {
        return FileType::new("application/x-gzip", "GZipped file.")
    } else {
        return FileType::new("unknown", "unknown")
    }
}