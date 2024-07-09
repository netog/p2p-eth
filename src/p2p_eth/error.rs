use std::num::TryFromIntError;

use ethereum_types::{H128, H256};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid tag received {0}")]
    InvalidTag(H256),

    #[error("Invalid Mac received {0}")]
    InvalidMac(H128),

    #[error("Invalid input {0}")]
    InvalidInput(String),

    #[error("concat_kdf error {0}")]
    ConcatKdf(String),

    #[error("secp256k1 error: {0}")]
    Secp256k1(String),

    #[error("TryFromIntError: {0}")]
    TryFromInt(#[from] TryFromIntError),

    #[error("Aes: invalid length")]
    AesInvalidLength(#[from] aes::cipher::InvalidLength),
}
