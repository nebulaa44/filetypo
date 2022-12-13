// the filetypo test suite

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

        assert_eq!(mime, "application/gzip");
    }

    #[test]
    fn jpeg_test()
    {
        let mut sample_gzip = File::open("./samples/jpeg-sample.jpeg").unwrap();
        let mime = guess::guess_type(&mut sample_gzip).mime;

        assert_eq!(mime, "image/jpeg");
    }

    #[test]
    fn mp3_test()
    {
        let mut sample_mp3 = File::open("./samples/mp3-sample.mp3").unwrap();
        let mime = guess::guess_type(&mut sample_mp3).mime;

        assert_eq!(mime, "audio/mpeg");
    }

    #[test]
    fn ogg_test()
    {
        let mut sample_ogg = File::open("./samples/ogg-sample.ogg").unwrap();
        let mime = guess::guess_type(&mut sample_ogg).mime;

        assert_eq!(mime, "audio/ogg");
    }

    #[test]
    fn png_test()
    {
        let mut sample_png = File::open("./samples/png-sample.png").unwrap();
        let mime = guess::guess_type(&mut sample_png).mime;

        assert_eq!(mime, "image/png");
    }

    #[test]
    fn truetype_test()
    {
        let mut sample_ttf = File::open("./samples/truetype-sample.ttf").unwrap();
        let mime = guess::guess_type(&mut sample_ttf).mime;

        assert_eq!(mime, "font/ttf")
    }

    #[test]
    fn wav_test()
    {
        let mut sample_wav = File::open("./samples/wav-sample.wav").unwrap();
        let mime = guess::guess_type(&mut sample_wav).mime;

        assert_eq!(mime, "audio/x-wav");
    }

}