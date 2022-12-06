mod conjugate_gradients;
pub mod fredholm_first_kind;
pub mod volterra_second_kind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    FunctionError(String),
}

use crate::function::table_function::Error as TableFunctionError;

impl From<TableFunctionError> for Error {
    fn from(e: TableFunctionError) -> Self {
        Self::FunctionError(format!("{:?}", e))
    }
}
