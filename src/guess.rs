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
    #[allow(dead_code)]
    pub fn new(mime: &str, description: &str) -> Self
    {
        Self {mime: mime.to_string(), description: description.to_string()}
    }
}

#[allow(dead_code)]
pub fn guess_type(file: &mut File) -> FileType 
{
    let mut file_bytes: Box<[u8]> = Box::new([0; 16]);
    file.read(&mut file_bytes).unwrap();

    // GZip archives
    if &file_bytes[0..=1] == b"\x1F\x8B" 
    {
        return FileType::new("application/gzip", "GZipped file.")
    }

    // MP3 Files (with ID3 data)
    else if &file_bytes[0..=2] == b"ID3"
    {
        return FileType::new("audio/mpeg", "MPEG audio with ID3 data")
    }

    // OGG Vorbis Files
    else if &file_bytes[0..=3] == b"OggS"
    {
        return FileType::new("audio/ogg", "Ogg Vorbis File")
    }

    // PNG files
    else if &file_bytes[0..=7] == b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"
    {
        return FileType::new("image/png", "PNG image")
    }

    else 
    {
        return FileType::default()
    }
}