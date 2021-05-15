//! Contains items for dealing with the `"data"` chunk of wave files.

use std::convert::TryFrom;

/// Enum listing the supported bit-depths and containers for the samples at each depth.
#[derive(Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub enum BitDepth {
    Eight(Vec<u8>),
    Sixteen(Vec<i16>),
    TwentyFour(Vec<i32>),
    ThirtyTwoFloat(Vec<f32>),
    Empty,
}

impl BitDepth {
    /// Returns `true` if the bit depth is [`Self::Eight`].
    #[must_use]
    pub fn is_eight(&self) -> bool {
        matches!(self, Self::Eight(..))
    }

    #[must_use]
    pub fn as_eight(&self) -> Option<&Vec<u8>> {
        if let Self::Eight(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_eight(self) -> Result<Vec<u8>, Self> {
        if let Self::Eight(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the bit depth is [`Self::Sixteen`].
    #[must_use]
    pub fn is_sixteen(&self) -> bool {
        matches!(self, Self::Sixteen(..))
    }

    #[must_use]
    pub fn as_sixteen(&self) -> Option<&Vec<i16>> {
        if let Self::Sixteen(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_sixteen(self) -> Result<Vec<i16>, Self> {
        if let Self::Sixteen(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the bit depth is [`Self::TwentyFour`].
    #[must_use]
    pub fn is_twenty_four(&self) -> bool {
        matches!(self, Self::TwentyFour(..))
    }

    #[must_use]
    pub fn as_twenty_four(&self) -> Option<&Vec<i32>> {
        if let Self::TwentyFour(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_twenty_four(self) -> Result<Vec<i32>, Self> {
        if let Self::TwentyFour(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the bit depth is [`Self::ThirtyTwoFloat`].
    #[must_use]
    pub fn is_thirty_two_float(&self) -> bool {
        matches!(self, Self::ThirtyTwoFloat(..))
    }

    #[must_use]
    pub fn as_thirty_two_float(&self) -> Option<&Vec<f32>> {
        if let Self::ThirtyTwoFloat(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_thirty_two_float(self) -> Result<Vec<f32>, Self> {
        if let Self::ThirtyTwoFloat(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the bit depth is [`Empty`].
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
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

impl From<Vec<f32>> for BitDepth {
    fn from(v: Vec<f32>) -> Self {
        BitDepth::ThirtyTwoFloat(v)
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

impl TryFrom<BitDepth> for Vec<f32> {
    type Error = &'static str;

    /// ## Errors
    ///
    /// This function fails if `value` is not `BitDepth::ThirtyTwoFloat`.
    fn try_from(value: BitDepth) -> Result<Self, Self::Error> {
        if let BitDepth::ThirtyTwoFloat(v) = value {
            Ok(v)
        } else {
            Err("Bit-depth is not 32bit float")
        }
    }
}
