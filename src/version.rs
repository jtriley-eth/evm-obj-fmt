use crate::error::Error;

/// EOF Version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// Version 1
    V1,
}

impl TryFrom<u8> for Version {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Version::V1),
            _ => Err(Error::InvalidVersion),
        }
    }
}

impl From<Version> for u8 {
    fn from(version: Version) -> Self {
        match version {
            Version::V1 => 1,
        }
    }
}
