use thiserror::Error;

// TODO: Reorganizes errors
#[derive(Error, PartialEq, Clone, Debug)]
pub enum Error {
    #[error("the current window size is invalid")]
    InvalidWindowSize,
    #[error("key or click was not sent due to the window not focused or other error")]
    KeyNotSent,
    #[error("window matching provided class and title cannot be found")]
    WindowNotFound,
    #[error("capture frame is not available")]
    FrameNotAvailable,
    #[error("key not found")]
    KeyNotFound,
    #[error("win32 API error {0}: {1}")]
    Win32(u32, String),
}

impl Error {
    #[inline]
    pub(crate) fn from_last_win_error() -> Error {
        Error::from(windows::core::Error::from_win32())
    }
}

impl From<windows::core::Error> for Error {
    fn from(error: windows::core::Error) -> Self {
        Error::Win32(error.code().0 as u32, error.message())
    }
}
