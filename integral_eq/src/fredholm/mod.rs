mod conjugate_gradients;
mod first_kind;
mod second_kind;

pub use first_kind::*;
pub use second_kind::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FunctionError(String),
}
