#![feature(test)]
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use wav::read;

extern crate test;
use test::{black_box, Bencher};

fn read_bench(b: &mut Bencher, data: &[u8]) {
    b.iter(|| {
        black_box(read(&mut Cursor::new(data))).unwrap();
    })
}

macro_rules! create_read_bench {
    ($name: ident , $path: literal) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut data = Vec::new();
            File::open($path).unwrap().read_to_end(&mut data).unwrap();
            read_bench(b, data.as_slice());
        }
    };
}

create_read_bench!(read_sine, "data/sine.wav");
create_read_bench!(read_sine_8bit_48khz, "data/sine_8bit48kHz.wav");
create_read_bench!(read_sine_16bit_48khz, "data/sine_16bit48kHz.wav");
create_read_bench!(read_sine_24bit_48khz, "data/sine_24bit48kHz.wav");
create_read_bench!(read_sine_32bit_float_48khz, "data/sine_32bitfloat48kHz.wav");

/*#[bench]
fn read_sine(b: &mut Bencher){
    let mut data = Vec::new();
    File::open(SINE_PATH).unwrap().read_to_end(&mut data).unwrap();
    read_bench(b , data.as_slice());
}

#[bench]
fn read_sine_8_48(b: &mut Bencher){
    let mut data = Vec::new();
    File::open(SINE_8_48_PATH).unwrap().read_to_end(&mut data).unwrap();
    b.iter(||{
        black_box(read(&mut Cursor::new(&data))).unwrap();
    })
}
#[bench]
fn read_sine_8_48(b: &mut Bencher){
    let mut data = Vec::new();
    File::open(SINE_8_48_PATH).unwrap().read_to_end(&mut data).unwrap();
    b.iter(||{
        black_box(read(&mut Cursor::new(&data))).unwrap();
    })
}*/
