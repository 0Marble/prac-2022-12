use crate::{
    integral_eq::fredholm_first_kind::fredholm_1st_system,
    mathparse::{DefaultRuntime, Expression},
};

use super::{
    form::Form,
    graph::{Graph, Path, PathKind},
    validate_expr, validate_from_str, Problem, ProblemCreator, Solution, SolutionParagraph,
    ValidationError,
};

pub struct Fredholm1stProblem {
    kernel: Box<dyn Expression>,
    right_side: Box<dyn Expression>,
    from: f64,
    to: f64,
    eps: f64,
    n: usize,
    max_iter_count: usize,
}

impl Problem for Fredholm1stProblem {
    fn solve(&self) -> Solution {
        let res = fredholm_1st_system(
            &|x, s| {
                self.kernel
                    .eval(&DefaultRuntime::new(&[("x", x), ("s", s)]))
            },
            &|x| self.right_side.eval(&DefaultRuntime::new(&[("x", x)])),
            self.from,
            self.to,
            self.n,
            self.eps,
            self.max_iter_count,
        );

        match res {
            Ok(res) => {
                match Graph::new(vec![Path {
                    pts: res.to_table(),
                    kind: PathKind::Line,
                    color: (1.0, 0.0, 0.0),
                }]) {
                    Some(g) => Solution {
                        explanation: vec![SolutionParagraph::Graph(g)],
                    },
                    None => Solution {
                        explanation: vec![SolutionParagraph::RuntimeError(
                            "Could not draw a graph".to_string(),
                        )],
                    },
                }
            }
            Err(e) => Solution {
                explanation: vec![SolutionParagraph::RuntimeError(format!("{:?}", e))],
            },
        }
    }
}

pub struct Fredholm1stProblemCreator {
    form: Form,
}

impl Default for Fredholm1stProblemCreator {
    fn default() -> Self {
        let mut form = Form::new(vec![
            "kernel".to_string(),
            "right_side".to_string(),
            "from".to_string(),
            "to".to_string(),
            "eps".to_string(),
            "n".to_string(),
            "max_iter_count".to_string(),
        ]);

        form.set("kernel", "abs(x-s)".to_string());
        form.set("right_side", "pow(x,2)".to_string());
        form.set("from", "-1".to_string());
        form.set("to", "1".to_string());
        form.set("eps", "1e-8".to_string());
        form.set("n", "50".to_string());
        form.set("max_iter_count", "10000".to_string());

        Self { form }
    }
}

impl ProblemCreator for Fredholm1stProblemCreator {
    fn form_mut(&mut self) -> &mut Form {
        &mut self.form
    }

    fn form(&self) -> &Form {
        &self.form
    }

    fn try_create(&self) -> Result<Box<dyn Problem>, Vec<ValidationError>> {
        let mut kernel: Option<Box<dyn Expression>> = None;
        let mut right_side: Option<Box<dyn Expression>> = None;
        let mut from: Option<f64> = None;
        let mut to: Option<f64> = None;
        let mut eps: Option<f64> = None;
        let mut n: Option<usize> = None;
        let mut max_iter_count: Option<usize> = None;

        let mut errors = vec![];
        for (name, val) in self.form.get_fields() {
            let res = match name {
                "kernel" => validate_expr(
                    name,
                    val,
                    &["x", "s"],
                    &DefaultRuntime::default(),
                    &mut kernel,
                ),
                "right_side" => validate_expr(
                    name,
                    val,
                    &["x"],
                    &DefaultRuntime::default(),
                    &mut right_side,
                ),
                "from" => validate_from_str::<f64>(name, val, &mut from),
                "to" => validate_from_str::<f64>(name, val, &mut to),
                "eps" => validate_from_str::<f64>(name, val, &mut eps),
                "n" => validate_from_str::<usize>(name, val, &mut n),
                "max_iter_count" => validate_from_str::<usize>(name, val, &mut max_iter_count),
                _ => Err(ValidationError(format!(
                    "{name} - no such field (probably a devs error)"
                ))),
            };

            match res {
                Ok(_) => {}
                Err(e) => errors.push(e),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let kernel = kernel.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: kernel".to_string(),
            ))
        });
        let right_side = right_side.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: right_side".to_string(),
            ))
        });
        let from = from.ok_or_else(|| {
            errors.push(ValidationError("field was not supplied: from".to_string()))
        });
        let to = to
            .ok_or_else(|| errors.push(ValidationError("field was not supplied: to".to_string())));
        let eps = eps
            .ok_or_else(|| errors.push(ValidationError("field was not supplied: eps".to_string())));
        let n =
            n.ok_or_else(|| errors.push(ValidationError("field was not supplied: n".to_string())));
        let max_iter_count = max_iter_count.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: max_iter_count".to_string(),
            ))
        });

        if errors.is_empty() {
            Ok(Box::new(Fredholm1stProblem {
                kernel: kernel.unwrap(),
                right_side: right_side.unwrap(),
                from: from.unwrap(),
                to: to.unwrap(),
                eps: eps.unwrap(),
                n: n.unwrap(),
                max_iter_count: max_iter_count.unwrap(),
            }))
        } else {
            Err(errors)
        }
    }
}
