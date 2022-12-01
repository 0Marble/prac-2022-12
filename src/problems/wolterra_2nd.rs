use crate::{
    integral_eq::wolterra_second_kind::wolterra_2nd_system,
    mathparse::{DefaultRuntime, Expression},
};

use super::{
    form::Form,
    graph::{Graph, Path, PathKind},
    validate_expr, validate_from_str, Problem, ProblemCreator, Solution, SolutionParagraph,
    ValidationError,
};

pub struct Wolterra2ndProblem {
    kernel: Box<dyn Expression>,
    right_side: Box<dyn Expression>,
    from: f64,
    to: f64,
    lambda: f64,
    n: usize,
}

impl Problem for Wolterra2ndProblem {
    fn solve(&self) -> Solution {
        let res = wolterra_2nd_system(
            &|x, s| {
                self.kernel
                    .eval(&DefaultRuntime::new(&[("x", x), ("s", s)]))
            },
            &|x| self.right_side.eval(&DefaultRuntime::new(&[("x", x)])),
            self.from,
            self.to,
            self.lambda,
            self.n,
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

pub struct Wolterra2ndProblemCreator {
    form: Form,
}

impl Default for Wolterra2ndProblemCreator {
    fn default() -> Self {
        let mut form = Form::new(vec![
            "kernel".to_string(),
            "right_side".to_string(),
            "from".to_string(),
            "to".to_string(),
            "lambda".to_string(),
            "n".to_string(),
        ]);

        form.set("kernel", "exp(x-s)".to_string());
        form.set("right_side", "1".to_string());
        form.set("from", "0".to_string());
        form.set("to", "1".to_string());
        form.set("lambda", "1".to_string());
        form.set("n", "50".to_string());

        Self { form }
    }
}

impl ProblemCreator for Wolterra2ndProblemCreator {
    fn form_mut(&mut self) -> &mut Form {
        &mut self.form
    }

    fn form(&self) -> &Form {
        &self.form
    }

    fn try_create(&self) -> Result<Box<dyn Problem>, Vec<ValidationError>> {
        let mut kernel = None;
        let mut right_side = None;
        let mut from = None;
        let mut to = None;
        let mut lambda = None;
        let mut n = None;

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
                "n" => validate_from_str::<usize>(name, val, &mut n),
                "lambda" => validate_from_str::<f64>(name, val, &mut lambda),
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
        let n =
            n.ok_or_else(|| errors.push(ValidationError("field was not supplied: n".to_string())));
        let lambda = lambda.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: lambda".to_string(),
            ))
        });

        if errors.is_empty() {
            Ok(Box::new(Wolterra2ndProblem {
                kernel: kernel.unwrap(),
                right_side: right_side.unwrap(),
                from: from.unwrap(),
                to: to.unwrap(),
                n: n.unwrap(),
                lambda: lambda.unwrap(),
            }))
        } else {
            Err(errors)
        }
    }
}
