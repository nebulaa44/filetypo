use std::fs::File;

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

pub fn guess_type(file: &File) -> FileType 
{
    FileType::new("deez/deez", "placeholder")
}