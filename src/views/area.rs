use std::time::Instant;

use crate::area_calc::calc_area;
use crate::mathparse::{parse, DefaultRuntime};

use crate::views::DisplayedResult;

use super::{Error as ViewError, View};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    F1Field(String),
    F2Field(String),
    F3Field(String),
    X12FromField(String),
    X12ToField(String),
    X13FromField(String),
    X13ToField(String),
    X23FromField(String),
    X23ToField(String),
    AreaEpsField(String),
    MaxIterCountField(String),
    Calculation(String),
    F1IncorrectArgCount { expected: usize, got: usize },
    F2IncorrectArgCount { expected: usize, got: usize },
    F3IncorrectArgCount { expected: usize, got: usize },
}

impl From<Error> for ViewError {
    fn from(e: Error) -> Self {
        match e {
            Error::F1Field(e) => ViewError::InvalidField {
                name: "f1".to_string(),
                err: e,
            },
            Error::F2Field(e) => ViewError::InvalidField {
                name: "f2".to_string(),
                err: e,
            },
            Error::F3Field(e) => ViewError::InvalidField {
                name: "f3".to_string(),
                err: e,
            },
            Error::X12FromField(e) => ViewError::InvalidField {
                name: "x12_from".to_string(),
                err: e,
            },
            Error::X12ToField(e) => ViewError::InvalidField {
                name: "x12_to".to_string(),
                err: e,
            },
            Error::X13FromField(e) => ViewError::InvalidField {
                name: "x13_from".to_string(),
                err: e,
            },
            Error::X13ToField(e) => ViewError::InvalidField {
                name: "x13_to".to_string(),
                err: e,
            },
            Error::X23FromField(e) => ViewError::InvalidField {
                name: "x23_from".to_string(),
                err: e,
            },
            Error::X23ToField(e) => ViewError::InvalidField {
                name: "x23_to".to_string(),
                err: e,
            },
            Error::AreaEpsField(e) => ViewError::InvalidField {
                name: "eps".to_string(),
                err: e,
            },
            Error::MaxIterCountField(e) => ViewError::InvalidField {
                name: "max_iter_count".to_string(),
                err: e,
            },
            Error::F1IncorrectArgCount { expected, got } => ViewError::InvalidField {
                name: "f1".to_string(),
                err: format!("Incorrect argument count: expected {expected}, got {got}"),
            },
            Error::F2IncorrectArgCount { expected, got } => ViewError::InvalidField {
                name: "f2".to_string(),
                err: format!("Incorrect argument count: expected {expected}, got {got}"),
            },
            Error::F3IncorrectArgCount { expected, got } => ViewError::InvalidField {
                name: "f3".to_string(),
                err: format!("Incorrect argument count: expected {expected}, got {got}"),
            },

            Error::Calculation(e) => ViewError::Runtime(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaView {
    f1_field: String,
    f2_field: String,
    f3_field: String,
    x12_from_field: String,
    x12_to_field: String,
    x13_from_field: String,
    x13_to_field: String,
    x23_from_field: String,
    x23_to_field: String,
    eps_field: String,
    max_iter_count_field: String,
}

impl Default for AreaView {
    fn default() -> Self {
        Self {
            f1_field: "exp(x)+2".to_string(),
            f2_field: "-2x+8".to_string(),
            f3_field: "-5/x".to_string(),
            x12_from_field: "0".to_string(),
            x12_to_field: "2".to_string(),
            x13_from_field: "-4".to_string(),
            x13_to_field: "-1".to_string(),
            x23_from_field: "-2".to_string(),
            x23_to_field: "-0.1".to_string(),
            eps_field: "0.001".to_string(),
            max_iter_count_field: "1000".to_string(),
        }
    }
}

impl View for AreaView {
    fn get_fields(&self) -> Vec<String> {
        vec![
            "f1".to_string(),
            "f2".to_string(),
            "f3".to_string(),
            "x12_from".to_string(),
            "x12_to".to_string(),
            "x13_from".to_string(),
            "x13_to".to_string(),
            "x23_from".to_string(),
            "x23_to".to_string(),
            "eps".to_string(),
            "max_iter_count".to_string(),
        ]
    }

    fn set_field(&mut self, name: &str, val: String) -> Result<(), ViewError> {
        match name {
            "f1" => self.f1_field = val,
            "f2" => self.f2_field = val,
            "f3" => self.f3_field = val,
            "x12_from" => self.x12_from_field = val,
            "x12_to" => self.x12_to_field = val,
            "x13_from" => self.x13_from_field = val,
            "x13_to" => self.x13_to_field = val,
            "x23_from" => self.x23_from_field = val,
            "x23_to" => self.x23_to_field = val,
            "eps" => self.eps_field = val,
            "max_iter_count" => self.max_iter_count_field = val,
            _ => unreachable!(),
        }

        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&str> {
        match name {
            "f1" => Some(&self.f1_field),
            "f2" => Some(&self.f2_field),
            "f3" => Some(&self.f3_field),
            "x12_from" => Some(&self.x12_from_field),
            "x12_to" => Some(&self.x12_to_field),
            "x13_from" => Some(&self.x13_from_field),
            "x13_to" => Some(&self.x13_to_field),
            "x23_from" => Some(&self.x23_from_field),
            "x23_to" => Some(&self.x23_to_field),
            "eps" => Some(&self.eps_field),
            "max_iter_count" => Some(&self.max_iter_count_field),
            _ => None,
        }
    }

    fn solve(&self) -> Result<Vec<DisplayedResult>, ViewError> {
        let lang = DefaultRuntime::default();

        let f1 = parse(&self.f1_field, &lang)
            .ok_or_else(|| Error::F1Field("Unable to parse f1".to_owned()))?;
        let f2 = parse(&self.f2_field, &lang)
            .ok_or_else(|| Error::F2Field("Unable to parse f2".to_owned()))?;
        let f3 = parse(&self.f3_field, &lang)
            .ok_or_else(|| Error::F3Field("Unable to parse f3".to_owned()))?;

        let x12_from = self
            .x12_from_field
            .parse::<f64>()
            .map_err(|e| Error::X12FromField(format!("{:?}", e)))?;
        let x12_to = self
            .x12_to_field
            .parse::<f64>()
            .map_err(|e| Error::X12ToField(format!("{:?}", e)))?;
        let x13_from = self
            .x13_from_field
            .parse::<f64>()
            .map_err(|e| Error::X13FromField(format!("{:?}", e)))?;
        let x13_to = self
            .x13_to_field
            .parse::<f64>()
            .map_err(|e| Error::X13ToField(format!("{:?}", e)))?;
        let x23_from = self
            .x23_from_field
            .parse::<f64>()
            .map_err(|e| Error::X23FromField(format!("{:?}", e)))?;
        let x23_to = self
            .x23_to_field
            .parse::<f64>()
            .map_err(|e| Error::X23ToField(format!("{:?}", e)))?;
        let eps = self
            .eps_field
            .parse::<f64>()
            .map_err(|e| Error::AreaEpsField(format!("{:?}", e)))?;
        let max_iter_count = self
            .max_iter_count_field
            .parse::<usize>()
            .map_err(|e| Error::MaxIterCountField(format!("{:?}", e)))?;

        fn minmax(a: f64, b: f64) -> (f64, f64) {
            if a < b {
                (a, b)
            } else {
                (b, a)
            }
        }

        let (x12_from, x12_to) = minmax(x12_from, x12_to);
        let (x13_from, x13_to) = minmax(x13_from, x13_to);
        let (x23_from, x23_to) = minmax(x23_from, x23_to);

        let f1_vars = f1.query_vars();
        if f1_vars.len() != 1 {
            return Err(Error::F1IncorrectArgCount {
                expected: 1,
                got: f1_vars.len(),
            })?;
        }

        let f2_vars = f2.query_vars();
        if f2_vars.len() != 1 {
            return Err(Error::F2IncorrectArgCount {
                expected: 1,
                got: f2_vars.len(),
            })?;
        }

        let f3_vars = f3.query_vars();
        if f3_vars.len() != 1 {
            return Err(Error::F3IncorrectArgCount {
                expected: 1,
                got: f3_vars.len(),
            })?;
        }

        let v1 = f1_vars.iter().next().unwrap().to_string();
        let v2 = f2_vars.iter().next().unwrap().to_string();
        let v3 = f3_vars.iter().next().unwrap().to_string();

        let (area, x12, x13, x23) = calc_area(
            &|x| f1.eval(&DefaultRuntime::new(&[(&v1, x)])),
            &|x| f2.eval(&DefaultRuntime::new(&[(&v2, x)])),
            &|x| f3.eval(&DefaultRuntime::new(&[(&v3, x)])),
            [x12_from, x12_to],
            [x13_from, x13_to],
            [x23_from, x23_to],
            0.001,
            eps,
            max_iter_count,
        )
        .map_err(|e| Error::Calculation(format!("{:?}", e)))?;

        Ok(vec![
            DisplayedResult::Text(format!(
                "Area = {area}, x12 = {x12}, x13 = {x13}, x23 = {x23}"
            )),
            DisplayedResult::Functions(vec![
                (
                    Box::new(move |x| {
                        f1.eval(&DefaultRuntime::new(&[(&v1, x)]))
                            .map_err(|e| format!("{:?}", e))
                    }),
                    f64::min(x12_from, x13_from),
                    f64::max(x12_to, x13_to),
                ),
                (
                    Box::new(move |x| {
                        f2.eval(&DefaultRuntime::new(&[(&v2, x)]))
                            .map_err(|e| format!("{:?}", e))
                    }),
                    f64::min(x12_from, x23_from),
                    f64::max(x12_to, x23_to),
                ),
                (
                    Box::new(move |x| {
                        f3.eval(&DefaultRuntime::new(&[(&v3, x)]))
                            .map_err(|e| format!("{:?}", e))
                    }),
                    f64::min(x13_from, x23_from),
                    f64::max(x13_to, x23_to),
                ),
            ]),
        ])
    }
}

#[test]
fn area() -> Result<(), ViewError> {
    let mut view = AreaView::default();
    let fields = vec![
        ("f1".to_string(), "1+4/(x*x+1)".to_string()),
        ("f2".to_string(), "pow(2,-x)".to_string()),
        ("f3".to_string(), "pow(x,3)".to_string()),
        ("x12_from".to_string(), "-2".to_string()),
        ("x12_to".to_string(), "-1".to_string()),
        ("x13_from".to_string(), "0.5".to_string()),
        ("x13_to".to_string(), "1.5".to_string()),
        ("x23_from".to_string(), "0.5".to_string()),
        ("x23_to".to_string(), "1.5".to_string()),
        ("eps".to_string(), "0.001".to_string()),
        ("max_iter_count".to_string(), "1000".to_string()),
    ];

    assert_eq!(
        view.get_fields(),
        fields
            .iter()
            .map(|(n, _)| n.to_string())
            .collect::<Vec<_>>()
    );

    fields
        .iter()
        .try_for_each(|(name, val)| view.set_field(name, val.to_owned()))?;
    assert!(fields
        .iter()
        .all(|(name, val)| view.get_field(name).map_or(false, |f| f == val)));

    let res = view.solve()?;
    if let DisplayedResult::Text(t) = &res[0] {
        assert_eq!(t, "Area = 6.5901592526739865, x12 = -1.3078612427555194, x13 = 1.343650541755197, x23 = 0.826214727131511");
    } else {
        unreachable!();
    }

    let actual_funcs = [
        (|x: f64| 1.0 + 4.0 / (x * x + 1.0)) as fn(f64) -> f64,
        (|x: f64| 2.0f64.powf(-x)) as fn(f64) -> f64,
        (|x: f64| x * x * x) as fn(f64) -> f64,
    ];

    let eps = 0.001;
    let n = 10;

    if let DisplayedResult::Functions(funcs) = &res[1] {
        for i in 0..3 {
            let (f, from, to) = &funcs[i];
            let actual = actual_funcs[i];
            let step = (to - from) / (n as f64);
            assert!((0..n)
                .map(|i| (i as f64) * step + from)
                .map(|x| (actual(x) - f.apply(x).unwrap()).abs())
                .all(|diff| diff < eps))
        }
    } else {
        unreachable!()
    }

    Ok(())
}
