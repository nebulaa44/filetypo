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
    #[allow(dead_code)]
    pub fn new(mime: &str, description: &str) -> Self
    {
        Self {mime: mime.to_string(), description: description.to_string()}
    }
}

impl Default for FileType
{
    fn default() -> FileType 
    {
        return FileType::new("unknown", "Unknown file type.");
    }
}

#[allow(dead_code)]
pub fn guess_type(file: &mut File) -> FileType 
{
    let mut file_bytes: Box<[u8]> = Box::new([0; 16]);
    file.read(&mut file_bytes).unwrap();

    // GZip archives
    if &file_bytes[0..2] == b"\x1F\x8B" 
    {
        return FileType::new("application/gzip", "GZipped file.")
    }

    // JPEG files
    if &file_bytes[0..3] == b"\xFF\xd8\xFF"
    {
        return FileType::new("image/jpeg", "JPEG image")
    }

    // MP3 Files (with ID3 data)
    else if &file_bytes[0..3] == b"ID3"
    {
        return FileType::new("audio/mpeg", "MPEG audio with ID3 data")
    }

    // OGG Vorbis Files
    else if &file_bytes[0..4] == b"OggS"
    {
        return FileType::new("audio/ogg", "Ogg Vorbis File")
    }

    // PNG files
    else if &file_bytes[0..8] == b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"
    {
        return FileType::new("image/png", "PNG image")
    }

    // WAV files
    else if &file_bytes[0..4] == b"RIFF" && &file_bytes[8..12] == b"WAVE"
    {
        return FileType::new("audio/x-wav", "WAV audio")
    }

    else 
    {
        return FileType::default()
    }
}