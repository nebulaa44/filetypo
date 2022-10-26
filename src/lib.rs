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

}