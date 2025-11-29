use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("failed to parse the IP address: {0}")]
    IpParseError(#[from] std::net::AddrParseError),

    #[error("failed to parse the network or CIDR mask: {0}")]
    MaskParseError(#[from] ipnet::AddrParseError),

    #[error("invalid network prefix length: {0}")]
    InvalidPrefixLength(#[from] ipnet::PrefixLenError),

    #[error("invalid prefix number: {0}")]
    InvalidPrefixNumber(#[from] std::num::ParseIntError),

    #[error("missing mask value")]
    MissingMask,
}
