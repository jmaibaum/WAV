use std::io;

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
            let mut inp: io::Cursor<Vec<u8>> = io::Cursor::new(raw.into());
            let mut out: io::Cursor<Vec<u8>> = io::Cursor::new(vec![0u8 ; raw.len()]);

            let (header, data) = wav::read(&mut inp).unwrap();

            assert_eq!(header.audio_format, $af);
            assert_eq!(header.channel_count, $cc);
            assert_eq!(header.sampling_rate, $sr);
            assert_eq!(header.bits_per_sample, $bi_p_sa);
            assert_eq!(header.bytes_per_sample, $by_p_sa);
            assert_eq!(header.bytes_per_second, $by_p_se);
            assert!($ibf(&data));

            wav::write(header, &data, &mut out).unwrap();

            for (i, o) in inp.into_inner().drain(0..).zip(out.into_inner().drain(0..)) {
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
