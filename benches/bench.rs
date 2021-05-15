#![feature(test)]
use std::io::Cursor;
use std::io::Read;
use std::{cell::Cell, fs::File};
use wav::{bit_depth::BitDepth, header::Header, read, write};

extern crate test;
use test::{black_box, Bencher};

fn read_bench(b: &mut Bencher, data: &[u8]) {
    b.iter(|| {
        black_box(read(&mut Cursor::new(data))).unwrap();
    })
}

// fn write_bench(b: &mut Bencher, data: Cell<&mut [u8]>, header: Header, bd: BitDepth) {
//     b.iter(|| {
//         black_box(write(header, &bd, &mut Cursor::new(data.take()))).unwrap();
//     })
// }

macro_rules! create_benches {
    ($({$read_name:ident, $write_name:ident, $path:literal $(,)*}),* $(,)*) => {$(
        #[bench]
        fn $read_name(b: &mut Bencher) {
            let mut inp = Vec::new();
            File::open($path).unwrap().read_to_end(&mut inp).unwrap();
            read_bench(b, &inp);
        }

        // #[bench]
        // fn $write_name(b: &mut Bencher) {
        //     let mut inp = Vec::new();
        //     File::open($path).unwrap().read_to_end(&mut inp).unwrap();
        //     let len = inp.len();
        //     let (header, track) = read(&mut Cursor::new(inp)).unwrap();
        //     let mut out = vec![0u8 ; len];
        //     write_bench(b, Cell::new(&mut out), header, track);
        // }
    )*};
}

create_benches! {
    {read_sine, write_sine, "data/sine.wav"},
    {read_sine_8bit_48khz, write_sine_8bit_48khz, "data/sine_8bit_48khz.wav"},
    {read_sine_16bit_48khz, write_sine_16bit_48khz, "data/sine_16bit_48khz.wav"},
    {read_sine_24it_48khz, write_sine_24bit_48khz, "data/sine_24bit_48khz.wav"},
    {read_sine_32bit_float_48khz, write_sine_32bit_float_48khz, "data/sine_32bit_float_48khz.wav"},
}
