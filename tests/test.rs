use std::io;

struct Data(Vec<u8>);

impl Data {
    fn reader(&self) -> EmulatedReader {
        EmulatedReader {
            bytes: &self.0,
            pos: 0,
        }
    }

    fn writer(&mut self) -> EmulatedWriter {
        EmulatedWriter {
            bytes: &mut self.0,
            pos: 0,
        }
    }
}

struct EmulatedReader<'a> {
    bytes: &'a[u8],
    pos: usize,
}

impl<'a> io::Read for EmulatedReader<'a> {
    fn read(&mut self, buf: &mut[u8]) -> io::Result<usize> {
        let mut num_read = 0;

        for (out, inp) in buf.iter_mut().zip(self.bytes[self.pos..].iter()) {
            *out = *inp;
            num_read += 1;
        }

        self.pos += num_read;
        Ok(num_read)
    }
}

impl<'a> io::Seek for EmulatedReader<'a> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match pos {
            io::SeekFrom::Start(n) => self.pos = n as usize,
            io::SeekFrom::End(n) => self.pos = self.bytes.len() - n as usize,
            io::SeekFrom::Current(n) => self.pos += n as usize,
        }

        Ok(self.pos as u64)
    }
}

struct EmulatedWriter<'a> {
    bytes: &'a mut[u8],
    pos: usize,
}

impl<'a> io::Write for EmulatedWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut num_written = 0;

        for (out, inp) in self.bytes[self.pos..].iter_mut().zip(buf.iter()) {
            *out = *inp;
            num_written += 1;
        }

        self.pos += num_written;
        Ok(num_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> io::Seek for EmulatedWriter<'a> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match pos {
            io::SeekFrom::Start(n) => self.pos = n as usize,
            io::SeekFrom::End(n) => self.pos = self.bytes.len() - n as usize,
            io::SeekFrom::Current(n) => self.pos += n as usize,
        }

        Ok(self.pos as u64)
    }
}

macro_rules! impl_wav_test {
    ( $(
        $F:ident {
            File: $f:expr,
            AudioFormat: $af:expr,
            ChannelCount: $cc:expr,
            SamplingRate: $sr:expr,
            BitsPerSample: $bi_p_sa:expr,
            BytesPerSample: $by_p_sa:expr,
            BytesPerSecond: $by_p_se:expr,
            IsBitsFn: $ibf:path
            $(,)*
        }
    ),* $(,)* ) => { $(
        #[test]
        fn $F() {
            let raw: &[u8] = include_bytes!($f);
            let mut inp = Data(raw.into());
            let mut out = Data(vec![0u8 ; raw.len()]);

            let (header, data) = wav::read(&mut inp.reader()).unwrap();

            assert_eq!(header.audio_format, $af);
            assert_eq!(header.channel_count, $cc);
            assert_eq!(header.sampling_rate, $sr);
            assert_eq!(header.bits_per_sample, $bi_p_sa);
            assert_eq!(header.bytes_per_sample, $by_p_sa);
            assert_eq!(header.bytes_per_second, $by_p_se);
            assert!($ibf(&data));

            wav::write(header, &data, &mut out.writer()).unwrap();

            for (i, o) in inp.0.drain(0..).zip(out.0.drain(0..)) {
                assert_eq!(i, o);
            }
        }
    )* };
}

impl_wav_test! {
    sine_8bit_48khz {
        File: "../data/sine_8bit_48khz.wav",
        AudioFormat: 0x01,
        ChannelCount: 2,
        SamplingRate: 48_000,
        BitsPerSample: 8,
        BytesPerSample: 2,
        BytesPerSecond: 96_000,
        IsBitsFn: wav::bit_depth::BitDepth::is_eight,
    },
    sine_16bit_48khz {
        File: "../data/sine_16bit_48khz.wav",
        AudioFormat: 0x01,
        ChannelCount: 2,
        SamplingRate: 48_000,
        BitsPerSample: 16,
        BytesPerSample: 4,
        BytesPerSecond: 192_000,
        IsBitsFn: wav::bit_depth::BitDepth::is_sixteen,
    },
    sine_24bit_48khz {
        File: "../data/sine_24bit_48khz.wav",
        AudioFormat: 0x01,
        ChannelCount: 2,
        SamplingRate: 48_000,
        BitsPerSample: 24,
        BytesPerSample: 6,
        BytesPerSecond: 288_000,
        IsBitsFn: wav::bit_depth::BitDepth::is_twenty_four,
    },
    sine_32bit_float_48khz {
        File: "../data/sine_32bit_float_48khz.wav",
        AudioFormat: 0x03,
        ChannelCount: 2,
        SamplingRate: 48_000,
        BitsPerSample: 32,
        BytesPerSample: 8,
        BytesPerSecond: 384_000,
        IsBitsFn: wav::bit_depth::BitDepth::is_thirty_two_float,
    },
}
