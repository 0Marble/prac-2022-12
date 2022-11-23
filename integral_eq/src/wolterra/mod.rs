mod first_kind;
mod second_kind;

use common::table_function::Error as TableFunctionError;
pub use first_kind::*;
pub use second_kind::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FunctionError(String),
}

impl From<TableFunctionError> for Error {
    fn from(e: TableFunctionError) -> Self {
        Error::FunctionError(format!("{:?}", e))
    }
}
