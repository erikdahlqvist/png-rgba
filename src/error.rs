#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    CouldNotReadFile,
    InvalidPngSignature,
    InvalidChecksum,
    NoDataChunksFound,
    NoHeaderChunkFound,
    HeaderLengthNot13,
    MultipleHeaderChunksFound,
    NonZeroCompressionType,
    NonZeroFilterType,
    InterlacingTypeGreaterThanOne,
    UnrecognizedFilterType(u8),
}