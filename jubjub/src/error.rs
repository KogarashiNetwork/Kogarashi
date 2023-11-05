#[derive(Debug)]
pub enum Error {
    /// Hex string too long
    HexStringTooLong,
    /// Hex string invalid
    HexStringInvalid,
    /// Bytes too long
    BytesTooLong,
    /// Invalid byte
    BytesInvalid,
}
