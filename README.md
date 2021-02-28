# WAV

This is a crate for reading in and writing out wave files. It supports
uncompressed PCM bit depths of 8, 16, 24 bits, and 32bit IEEE Float formats,
both with any number of channels, Unfortunately other types of data format
(e.g. compressed WAVE files) are not supported. There is also no support for
any metadata chunks or any chunks other than the "fmt " and "data" chunks.

## Example

```rust
use std::fs::File;
use std::path::Path;

let mut inp_file = File::open(Path::new("data/sine.wav"))?;
let (header, data) = wav::read(&mut inp_file)?;

let mut out_file = File::create(Path::new("data/output.wav"))?;
wav::write(header, &data, &mut out_file)?;
```
