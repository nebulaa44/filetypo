mod guess;

#[cfg(test)]
mod tests 
{
    use std::fs::File;
    use super::*;

    #[test]
    fn gzip_test() 
    {
        let mut sample_gzip = File::open("./samples/gzip-sample.gz").unwrap();
        let mime = guess::guess_type(&mut sample_gzip).mime;

        assert_eq!(mime, "application/x-gzip");
    }

    #[test]
    fn mp3_test()
    {
        let mut sample_mp3 = File::open("./samples/mp3-sample.mp3").unwrap();
        let mime = guess::guess_type(&mut sample_mp3).mime;

        assert_eq!(mime, "audio/mpeg");
    }

    #[test]
    fn png_test()
    {
        let mut sample_png = File::open("./samples/png-sample.png").unwrap();
        let mime = guess::guess_type(&mut sample_png).mime;

        assert_eq!(mime, "image/png");
    }

}