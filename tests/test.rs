use std::fs::File;

#[test]
fn sine_8bit_48khz() {
    let mut reader = File::open(std::path::Path::new("data/sine_8bit_48khz.wav")).unwrap();
    let (h, b) = wav::read(&mut reader).unwrap();

    let mut writer = File::create(std::path::Path::new("data/output_8bit_48khz.wav")).unwrap();
    wav::write(h, &b, &mut writer).unwrap();
}

#[test]
fn sine_16bit_48khz() {
    let mut reader = File::open(std::path::Path::new("data/sine_16bit_48khz.wav")).unwrap();
    let (h, b) = wav::read(&mut reader).unwrap();

    let mut writer = File::create(std::path::Path::new("data/output_16bit_48khz.wav")).unwrap();
    wav::write(h, &b, &mut writer).unwrap();
}

#[test]
fn sine_24bit_48khz() {
    let mut reader = File::open(std::path::Path::new("data/sine_24bit_48khz.wav")).unwrap();
    let (h, b) = wav::read(&mut reader).unwrap();

    let mut writer = File::create(std::path::Path::new("data/output_24bit_48khz.wav")).unwrap();
    wav::write(h, &b, &mut writer).unwrap();
}

#[test]
fn sine_32bit_float_48khz() {
    let mut reader = File::open(std::path::Path::new("data/sine_32bit_float_48khz.wav")).unwrap();
    let (h, b) = wav::read(&mut reader).unwrap();

    let mut writer = File::create(std::path::Path::new("data/output_32bit_float_48khz.wav")).unwrap();
    wav::write(h, &b, &mut writer).unwrap();
}
