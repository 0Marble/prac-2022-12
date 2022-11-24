pub mod area;
pub mod golden_ratio_min;
pub mod integral_fredholm_1;
pub mod integral_wolterra_2;
pub mod penalty_min;

use std::path::PathBuf;

use crate::common::function::{Function, FunctionNd};

pub enum DisplayedResult {
    Text(String),
    FunctionNDim {
        f: Box<dyn FunctionNd<Error = String>>,
        from: Vec<f64>,
        to: Vec<f64>,
    },
    TextFile {
        path: PathBuf,
        contents: String,
    },
    Functions(Vec<(Box<dyn Function<Error = String>>, f64, f64)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidField { name: String, err: String },
    Runtime(String),
}

pub trait View {
    fn get_fields(&self) -> Vec<String>;
    fn get_field(&self, name: &str) -> Option<&str>;
    fn set_field(&mut self, name: &str, val: String) -> Result<(), Error>;
    fn solve(&self) -> Result<Vec<DisplayedResult>, Error>;
}
