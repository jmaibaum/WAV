# WAV

## Version 0.6.0

* Added benchmark tests under the `benches` folder [faddde7](https://github.com/Fluhzar/WAV/commit/faddde76cece1be530653d9e305fcad798627d54)
* Added support for 32-bit IEEE floating point data types [1117830](https://github.com/Fluhzar/WAV/commit/111783032030900464ead46be5c8924cac67e114)

## Version 0.5.0

* Modified the signature of `wav::write` to borrow the `BitDepth` parameter rather than own.
* Updated `riff` dependency to `^1.0`.
* Refactored code to be a bit more clear.
* Fixed incorrect endian-ness of the 24-bit reading and writing.
* Changed various `Into` and `TryInto` impls to `From` and `TryFrom` respectively.
* Updated a `From` impl that could panic into a `TryFrom` to prevent said panic.

## Version 0.4.1

* Fixed issue of using index-access of chunk instead of iterator, causing issues if there are chunks between "fmt " and "data"

## Version 0.4.0

* Changed `read_wav` to `read` and `write_wav` to `write`.
* Updated documentation.

## Version 0.3.0

* Change `read_wav` and `write_wav` to accept generic IO types.
* Changed an error message.

## Version 0.2.0

* Renamed `read_file` to `read_wav` for consistency and clarity.

## Version 0.1.1

* Gives proper error when audio data that isn't in uncompressed PCM format.
* Ensures it's finds the correct chunks for header and data info, ignoring any extra metadata chunks.
* Minor documentation updates.
