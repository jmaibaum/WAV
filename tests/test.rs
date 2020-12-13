extern crate wav;

#[cfg(test)]
mod tests {
    #[test]
    fn test_wav() -> std::io::Result<()> {
        use std::fs::File;

        let mut reader = File::open(std::path::Path::new("data/sine_8bit48kHz.wav"))?;
        let (h, b) = wav::read(&mut reader)?;

        let mut writer = File::create(std::path::Path::new("data/output_8bit48kHz.wav"))?;
        wav::write(h, b, &mut writer)?;

        let mut reader = File::open(std::path::Path::new("data/sine_16bit48kHz.wav"))?;
        let (h, b) = wav::read(&mut reader)?;

        let mut writer = File::create(std::path::Path::new("data/output_16bit48kHz.wav"))?;
        wav::write(h, b, &mut writer)?;

        let mut reader = File::open(std::path::Path::new("data/sine_24bit48kHz.wav"))?;
        let (h, b) = wav::read(&mut reader)?;

        let mut writer = File::create(std::path::Path::new("data/output_24bit48kHz.wav"))?;
        wav::write(h, b, &mut writer)?;

        Ok(())
    }
}
