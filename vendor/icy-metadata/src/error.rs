//! Errors returned from parsing icy metadata.

use std::error::Error;
use std::fmt::Display;
use std::string::FromUtf8Error;

/// Error returned when parsing metadata from a stream fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetadataParseError {
    /// Metadata block contained invalid UTF-8 data.
    InvalidUtf8(FromUtf8Error),
    /// Metadata block contained no valid values.
    Empty(EmptyMetadataError),
}

impl Display for MetadataParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Failed to parse icy metadata block as a string. The stream may not be properly \
             encoded.",
        )
    }
}

impl Error for MetadataParseError {}

/// Error returned when a metadata block contains no valid values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyMetadataError(pub String);

impl Display for EmptyMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No valid values found for metadata block {}", self.0)
    }
}

impl Error for EmptyMetadataError {}
