/// Enum listing the supported bit-depths and containers for the samples at each depth.
#[derive(Debug, PartialEq, Clone)]
pub enum BitDepth {
    Eight(Vec<u8>),
    Sixteen(Vec<i16>),
    TwentyFour(Vec<i32>),
    Empty,
}

impl Default for BitDepth {
    /// Default construction.
    fn default() -> Self {
        BitDepth::Empty
    }
}

impl From<Vec<u8>> for BitDepth {
    /// Creates a BitDepth object from the given u8 vector.
    fn from(v: Vec<u8>) -> Self {
        BitDepth::Eight(v)
    }
}
impl From<Vec<i16>> for BitDepth {
    /// Creates a BitDepth object from the given i16 vector.
    fn from(v: Vec<i16>) -> Self {
        BitDepth::Sixteen(v)
    }
}
impl From<Vec<i32>> for BitDepth {
    /// Creates a BitDepth object from the given i32 vector.
    fn from(v: Vec<i32>) -> Self {
        BitDepth::TwentyFour(v)
    }
}

impl std::convert::TryInto<Vec<u8>> for BitDepth {
    type Error = &'static str;

    /// Attempts to create a vector from the object.
    ///
    /// # Errors
    ///
    /// This function fails if `self` is not `BitDepth::Eight`.
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if let BitDepth::Eight(v) = self {
            Ok(v)
        } else {
            Err("Bit depth is not 8")
        }
    }
}
impl std::convert::TryInto<Vec<i16>> for BitDepth {
    type Error = &'static str;

    /// Attempts to create a vector from the object.
    ///
    /// # Errors
    ///
    /// This function fails if `self` is not `BitDepth::Sixteen`.
    fn try_into(self) -> Result<Vec<i16>, Self::Error> {
        if let BitDepth::Sixteen(v) = self {
            Ok(v)
        } else {
            Err("Bit depth is not 16")
        }
    }
}
impl std::convert::TryInto<Vec<i32>> for BitDepth {
    type Error = &'static str;

    /// Attempts to create a vector from the object.
    ///
    /// # Errors
    ///
    /// This function fails if `self` is not `BitDepth::TwentyFour`.
    fn try_into(self) -> Result<Vec<i32>, Self::Error> {
        if let BitDepth::TwentyFour(v) = self {
            Ok(v)
        } else {
            Err("Bit depth is not 24")
        }
    }
}
