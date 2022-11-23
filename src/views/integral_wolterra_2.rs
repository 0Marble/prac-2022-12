use std::convert::TryFrom;
use std::path::PathBuf;
use std::{fmt::Write, iter::FromIterator};

use common::function::Function;
use integral_eq::wolterra::wolterra_2nd_system;
use mathparse::{parse, DefaultRuntime};

use super::{DisplayedResult, Error as ViewError, View};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    KernelField(String),
    RightSideField(String),
    FromField(String),
    ToField(String),
    NField(String),
    LambdaField(String),
    SaveFilePathField(String),
    KernelInvalidVarCount {
        expected: usize,
        got: usize,
    },
    RightSideInvalidVarCount {
        expected: usize,
        got: usize,
    },
    KernelInvalidVarNames {
        expected: Vec<String>,
        got: Vec<String>,
    },
    Calculation(String),
}

impl From<Error> for ViewError {
    fn from(e: Error) -> Self {
        match e {
            Error::KernelField(e) => ViewError::InvalidField {
                name: "kernel".to_string(),
                err: e,
            },
            Error::RightSideField(e) => ViewError::InvalidField {
                name: "right_side".to_string(),
                err: e,
            },
            Error::FromField(e) => ViewError::InvalidField {
                name: "from".to_string(),
                err: e,
            },
            Error::ToField(e) => ViewError::InvalidField {
                name: "to".to_string(),
                err: e,
            },
            Error::NField(e) => ViewError::InvalidField {
                name: "lambda".to_string(),
                err: e,
            },
            Error::LambdaField(e) => ViewError::InvalidField {
                name: "n".to_string(),
                err: e,
            },
            Error::SaveFilePathField(e) => ViewError::InvalidField {
                name: "save_file_path".to_string(),
                err: e,
            },
            Error::KernelInvalidVarCount { expected, got } => ViewError::InvalidField {
                name: "kernel".to_string(),
                err: format!("Invalid argument count, expected {expected}, got {got}"),
            },
            Error::RightSideInvalidVarCount { expected, got } => ViewError::InvalidField {
                name: "right_side".to_string(),
                err: format!("Invalid argument count, expected {expected}, got {got}"),
            },
            Error::KernelInvalidVarNames { expected, got } => ViewError::InvalidField {
                name: "kernel".to_string(),
                err: format!(
                    "Invalid variable names, expected {:?}, got {:?}",
                    expected, got
                ),
            },
            Error::Calculation(e) => ViewError::Runtime(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Wolterra2View {
    kernel_field: String,
    right_side_field: String,
    from_field: String,
    to_field: String,
    lambda_field: String,
    n_field: String,
    save_file_path_field: String,
}

impl View for Wolterra2View {
    fn get_fields(&self) -> Vec<String> {
        vec![
            "kernel".to_string(),
            "right_side".to_string(),
            "from".to_string(),
            "to".to_string(),
            "lambda".to_string(),
            "n".to_string(),
            "save_file_path".to_string(),
        ]
    }

    fn set_field(&mut self, name: &str, val: String) -> Result<(), ViewError> {
        match name {
            "kernel" => self.kernel_field = val,
            "right_side" => self.right_side_field = val,
            "from" => self.from_field = val,
            "to" => self.to_field = val,
            "lambda" => self.lambda_field = val,
            "n" => self.n_field = val,
            "save_file_path" => self.save_file_path_field = val,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&str> {
        match name {
            "kernel" => Some(&self.kernel_field),
            "right_side" => Some(&self.right_side_field),
            "from" => Some(&self.from_field),
            "to" => Some(&self.to_field),
            "lambda" => Some(&self.lambda_field),
            "n" => Some(&self.n_field),
            "save_file_path" => Some(&self.save_file_path_field),
            _ => None,
        }
    }

    fn solve(&self) -> Result<Vec<super::DisplayedResult>, ViewError> {
        let lang = DefaultRuntime::default();
        let kernel = parse(&self.kernel_field, &lang)
            .ok_or_else(|| Error::KernelField("Could not parse kernel".to_owned()))?;
        let right_side = parse(&self.right_side_field, &lang)
            .ok_or_else(|| Error::RightSideField("Could not parse right side".to_owned()))?;

        let from = self
            .from_field
            .parse::<f64>()
            .map_err(|e| Error::FromField(format!("{:?}", e)))?;
        let to = self
            .to_field
            .parse::<f64>()
            .map_err(|e| Error::ToField(format!("{:?}", e)))?;
        let n = self
            .n_field
            .parse::<usize>()
            .map_err(|e| Error::NField(format!("{:?}", e)))?;
        let lambda = self
            .lambda_field
            .parse::<f64>()
            .map_err(|e| Error::LambdaField(format!("{:?}", e)))?;

        let kernel_vars = kernel.query_vars();
        if kernel_vars.len() != 2 {
            return Err(Error::KernelInvalidVarCount {
                expected: 2,
                got: kernel_vars.len(),
            })?;
        }

        let right_side_vars = right_side.query_vars();
        if right_side_vars.len() != 1 {
            return Err(Error::RightSideInvalidVarCount {
                expected: 1,
                got: right_side_vars.len(),
            })?;
        }

        let outside_var = right_side_vars.iter().copied().next().unwrap();
        if kernel_vars.iter().all(|v| v.ne(&outside_var)) {
            return Err(Error::KernelInvalidVarNames {
                expected: vec![outside_var.to_string(), "Any other var".to_string()],
                got: Vec::from_iter(kernel_vars.iter().copied().map(str::to_string)),
            })?;
        }
        let inside_var = kernel_vars
            .iter()
            .copied()
            .find(|v| v.ne(&outside_var))
            .unwrap();

        let func = wolterra_2nd_system(
            &|x, s| kernel.eval(&DefaultRuntime::new(&[(outside_var, x), (inside_var, s)])),
            &|x| right_side.eval(&DefaultRuntime::new(&[(outside_var, x)])),
            from,
            to,
            lambda,
            n,
        )
        .map_err(|e| Error::Calculation(format!("{:?}", e)))?;

        Ok(vec![
            DisplayedResult::TextFile {
                path: PathBuf::try_from(&self.save_file_path_field)
                    .map_err(|e| Error::SaveFilePathField(format!("{:?}", e)))?,
                contents: func.to_table().into_iter().try_fold(
                    String::new(),
                    |mut acc, (x, y)| -> Result<String, Error> {
                        writeln!(&mut acc, "{x},{y}")
                            .map_err(|e| Error::Calculation(format!("{:?}", e)))?;
                        Ok(acc)
                    },
                )?,
            },
            DisplayedResult::Function {
                f: Box::new(move |x| func.apply(x).map_err(|e| format!("{:?}", e))),
                from,
                to,
            },
        ])
    }
}
