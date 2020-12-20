use std::convert::TryFrom;

/// Enum listing the supported bit-depths and containers for the samples at each depth.
#[derive(Debug, PartialEq, Clone)]
pub enum BitDepth {
    Eight(Vec<u8>),
    Sixteen(Vec<i16>),
    TwentyFour(Vec<i32>),
    Empty,
}

impl Default for BitDepth {
    fn default() -> Self {
        BitDepth::Empty
    }
}

impl From<Vec<u8>> for BitDepth {
    fn from(v: Vec<u8>) -> Self {
        BitDepth::Eight(v)
    }
}

impl From<Vec<i16>> for BitDepth {
    fn from(v: Vec<i16>) -> Self {
        BitDepth::Sixteen(v)
    }
}

impl From<Vec<i32>> for BitDepth {
    fn from(v: Vec<i32>) -> Self {
        BitDepth::TwentyFour(v)
    }
}

impl TryFrom<BitDepth> for Vec<u8> {
    type Error = &'static str;

    /// ## Errors
    ///
    /// This function fails if `value` is not `BitDepth::Eight`.
    fn try_from(value: BitDepth) -> Result<Self, Self::Error> {
        if let BitDepth::Eight(v) = value {
            Ok(v)
        } else {
            Err("Bit-depth is not 8")
        }
    }
}

impl TryFrom<BitDepth> for Vec<i16> {
    type Error = &'static str;

    /// ## Errors
    ///
    /// This function fails if `value` is not `BitDepth::Sixteen`.
    fn try_from(value: BitDepth) -> Result<Self, Self::Error> {
        if let BitDepth::Sixteen(v) = value {
            Ok(v)
        } else {
            Err("Bit-depth is not 16")
        }
    }
}

impl TryFrom<BitDepth> for Vec<i32> {
    type Error = &'static str;

    /// ## Errors
    ///
    /// This function fails if `value` is not `BitDepth::TwentyFour`.
    fn try_from(value: BitDepth) -> Result<Self, Self::Error> {
        if let BitDepth::TwentyFour(v) = value {
            Ok(v)
        } else {
            Err("Bit-depth is not 24")
        }
    }
}
