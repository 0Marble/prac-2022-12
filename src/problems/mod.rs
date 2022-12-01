use std::{fmt::Debug, str::FromStr};

use crate::mathparse::{parse, Expression, Runtime};

use self::{form::Form, graph::Graph};

pub mod area_calc;
pub mod fredholm_1st;
pub mod wolterra_2nd;
pub mod penalty_min;

pub struct ValidationError(pub String);

pub mod graph;
#[derive(Debug)]
pub enum SolutionParagraph {
    Text(String),
    Graph(Graph),
    RuntimeError(String),
}

#[derive(Debug)]
pub struct Solution {
    pub explanation: Vec<SolutionParagraph>,
}

pub mod form;

pub trait Problem {
    fn solve(&self) -> Solution;
}

pub trait ProblemCreator {
    fn form(&self) -> &Form;
    fn form_mut(&mut self) -> &mut Form;
    fn try_create(&self) -> Result<Box<dyn Problem>, Vec<ValidationError>>;
}

fn validate_expr(
    field_name: &str,
    contents: &str,
    allowed_vars: &[&str],
    runtime: &dyn Runtime,
    expr: &mut Option<Box<dyn Expression>>,
) -> Result<(), ValidationError> {
    let res = match parse(contents, runtime) {
        Some(expr) => {
            let vars = expr.query_vars();
            if !vars.iter().all(|v| allowed_vars.iter().any(|a| a == v)) {
                Err(ValidationError(format!(
                    "{field_name} - vars {:?} not allowed, expected {:?}",
                    vars, allowed_vars
                )))
            } else {
                Ok(expr)
            }
        }
        None => Err(ValidationError(format!("{field_name} - could not parse"))),
    };

    match res {
        Ok(res) => {
            *expr = Some(res);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn validate_from_str<T>(
    field_name: &str,
    contents: &str,
    val: &mut Option<T>,
) -> Result<(), ValidationError>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    let res = match contents.parse::<T>() {
        Ok(t) => Ok(t),
        Err(e) => Err(ValidationError(format!(
            "{field_name} - could not parse: {:?}",
            e
        ))),
    };

    match res {
        Ok(res) => {
            *val = Some(res);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
