use super::{DisplayedResult, Error as ViewError, View};
use crate::common::function::Function;
use crate::integral_eq::fredholm::fredholm_1st_system;
use crate::mathparse::{parse, DefaultRuntime};
use std::str::FromStr;
use std::{convert::TryFrom, fmt::Write, iter::FromIterator, path::PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    KernelField(String),
    RightSideField(String),
    FromField(String),
    ToField(String),
    EpsField(String),
    NField(String),
    MaxIterCountField(String),
    SaveFilePathField(String),
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
            Error::EpsField(e) => ViewError::InvalidField {
                name: "eps".to_string(),
                err: e,
            },
            Error::NField(e) => ViewError::InvalidField {
                name: "n".to_string(),
                err: e,
            },
            Error::MaxIterCountField(e) => ViewError::InvalidField {
                name: "max_iter_count".to_string(),
                err: e,
            },
            Error::SaveFilePathField(e) => ViewError::InvalidField {
                name: "save_file_path".to_string(),
                err: e,
            },

            Error::Calculation(e) => ViewError::Runtime(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fredholm1View {
    kernel_field: String,
    right_side_field: String,
    from_field: String,
    to_field: String,
    eps_field: String,
    n_field: String,
    max_iter_count_field: String,
    save_file_path_field: String,
}

impl Default for Fredholm1View {
    fn default() -> Self {
        Self {
            kernel_field: "abs(x-s)".to_string(),
            right_side_field: "1+pow(x,2)".to_string(),
            from_field: "-1".to_string(),
            to_field: "1".to_string(),
            eps_field: "1e-8".to_string(),
            n_field: "50".to_string(),
            max_iter_count_field: "10000".to_string(),
            save_file_path_field: "./func.csv".to_string(),
        }
    }
}

impl View for Fredholm1View {
    fn get_fields(&self) -> Vec<String> {
        vec![
            "kernel".to_string(),
            "right_side".to_string(),
            "from".to_string(),
            "to".to_string(),
            "eps".to_string(),
            "n".to_string(),
            "max_iter_count".to_string(),
            "save_file_path".to_string(),
        ]
    }

    fn set_field(&mut self, name: &str, val: String) -> Result<(), ViewError> {
        match name {
            "kernel" => self.kernel_field = val,
            "right_side" => self.right_side_field = val,
            "from" => self.from_field = val,
            "to" => self.to_field = val,
            "eps" => self.eps_field = val,
            "n" => self.n_field = val,
            "max_iter_count" => self.max_iter_count_field = val,
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
            "eps" => Some(&self.eps_field),
            "n" => Some(&self.n_field),
            "max_iter_count" => Some(&self.max_iter_count_field),
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
        let eps = self
            .eps_field
            .parse::<f64>()
            .map_err(|e| Error::EpsField(format!("{:?}", e)))?;
        let n = self
            .n_field
            .parse::<usize>()
            .map_err(|e| Error::NField(format!("{:?}", e)))?;
        let max_iter_count = self
            .max_iter_count_field
            .parse::<usize>()
            .map_err(|e| Error::MaxIterCountField(format!("{:?}", e)))?;

        let kernel_vars = kernel.query_vars();
        let right_side_vars = right_side.query_vars();

        let outside_var = "x";
        let inside_var = "s";

        if kernel_vars
            .iter()
            .any(|v| v != &outside_var && v != &inside_var)
        {
            return Err(ViewError::InvalidField {
                name: "kernel".to_string(),
                err: format!(
                    "Invalid variable names, expected [{inside_var}, {outside_var}] got {:?}",
                    kernel_vars
                ),
            });
        }

        if right_side_vars.iter().any(|v| v != &outside_var) {
            return Err(ViewError::InvalidField {
                name: "right_side".to_string(),
                err: format!(
                    "Invalid variable names, expected {outside_var}, got {:?}",
                    kernel_vars
                ),
            });
        }

        let func = fredholm_1st_system(
            &|x, s| kernel.eval(&DefaultRuntime::new(&[(outside_var, x), (inside_var, s)])),
            &|x| right_side.eval(&DefaultRuntime::new(&[(outside_var, x)])),
            from,
            to,
            n,
            eps,
            max_iter_count,
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
            DisplayedResult::Functions(vec![(
                Box::new(move |x| func.apply(x).map_err(|e| format!("{:?}", e))),
                from,
                to,
            )]),
        ])
    }
}

#[test]
fn fredholm_view() -> Result<(), ViewError> {
    let mut view = Fredholm1View::default();
    let fields = vec![
        ("kernel".to_string(), "abs(x-s)".to_string()),
        ("right_side".to_string(), "1+pow(x,2)".to_string()),
        ("from".to_string(), "-1".to_string()),
        ("to".to_string(), "1".to_string()),
        ("eps".to_string(), "1e-8".to_string()),
        ("n".to_string(), "50".to_string()),
        ("max_iter_count".to_string(), "10000".to_string()),
        ("save_file_path".to_string(), "./func.csv".to_string()),
    ];
    assert_eq!(
        view.get_fields(),
        fields
            .iter()
            .map(|(n, _)| n.to_string())
            .collect::<Vec<_>>()
    );

    assert!(fields
        .iter()
        .try_for_each(|(name, val)| view.set_field(name, val.to_owned()))
        .is_ok());
    assert!(fields
        .iter()
        .all(|(name, val)| view.get_field(name).map_or(false, |f| f == val)));

    let res = view.solve()?;

    if let DisplayedResult::TextFile { path, contents: _ } = &res[0] {
        assert_eq!(path, &PathBuf::from_str("./func.csv").unwrap());
    } else {
        unreachable!()
    }

    let eps = 0.05;
    let n = 10;
    if let DisplayedResult::Functions(funcs) = &res[1] {
        let (f, from, to) = &funcs[0];
        let step = (to - from) / (n as f64);
        assert!((1..n)
            .map(|i| (i as f64) * step + from)
            .map(|x| (f.apply(x).unwrap() - 1.0).abs())
            .all(|diff| diff < eps));
    } else {
        unreachable!()
    }

    Ok(())
}
