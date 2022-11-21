#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    TableEmpty,
    PointOutOfBounds { x: f64, min: f64, max: f64 },
    IoError(String),
    InvalidCsv { line: usize },
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e.to_string())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Error::IoError(e.to_string())
    }
}
