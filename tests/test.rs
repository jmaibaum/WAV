extern crate wav;

#[cfg(test)]
mod tests {
    #[test]
    fn test_wav() {
        use std::fs::File;

        let mut reader = File::open(std::path::Path::new("data/sine.wav")).unwrap();
        let (h,b) = wav::read_wav(&mut reader).unwrap();

        let mut writer = File::create(std::path::Path::new("data/output.wav")).unwrap();
        wav::write_wav(h, b, &mut writer).unwrap();
    }
}
