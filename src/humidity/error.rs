#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Timeout during communication")]
    Timeout,
    #[error("CRC mismatch")]
    CrcMismatch,
    #[error(transparent)]
    GpioError(#[from] rppal::gpio::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
