extern crate wav;

#[cfg(test)]
mod tests {
	#[test]
	fn test_wav() {
		let (h,b) = wav::read_wav(std::path::Path::new("data/sine.wav")).unwrap();

		wav::write_wav(h, b, std::path::Path::new("data/output.wav")).unwrap();
	}
}
