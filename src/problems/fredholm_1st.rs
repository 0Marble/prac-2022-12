use std::{fs::File, io::Write};

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

struct Fredholm1stProblem {
    kernel: Box<dyn Expression>,
    right_side: Box<dyn Expression>,
    from: f64,
    to: f64,
    eps: f64,
    n: usize,
    max_iter_count: usize,
    dest_file: String,
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
                let mut solution = vec![];
                let kernel_latex = self.kernel.to_latex(&DefaultRuntime::default());
                let right_side_latex = self.right_side.to_latex(&DefaultRuntime::default());

                if let (Ok(kernel_latex), Ok(right_side_latex)) = (kernel_latex, right_side_latex) {
                    let latex = SolutionParagraph::Latex(format!(
                        "\\int_{{{}}}^{{{}}}{{{}}}y(s)ds={{{}}}",
                        self.from, self.to, kernel_latex, right_side_latex
                    ));
                    solution.push(latex);
                }

                let pts = res.to_table();
                let write_res = match File::create(&self.dest_file) {
                    Ok(mut file) => pts
                        .iter()
                        .try_for_each(|(x, y)| writeln!(file, "{},{}", x, y)),
                    Err(e) => Err(e),
                };

                let _ = write_res.map_err(|e| {
                    solution.push(SolutionParagraph::RuntimeError(format!("{:?}", e)))
                });

                match Graph::new(vec![Path {
                    pts,
                    kind: PathKind::Line,
                    color: (1.0, 0.0, 0.0),
                }]) {
                    Some(g) => solution.push(SolutionParagraph::Graph(g)),
                    None => solution.push(SolutionParagraph::RuntimeError(
                        "Could not draw a graph".to_string(),
                    )),
                }

                Solution {
                    explanation: solution,
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
            "dest_file".to_string(),
        ]);

        form.set("kernel", "abs(x-s)".to_string());
        form.set("right_side", "pow(x,2)".to_string());
        form.set("from", "-1".to_string());
        form.set("to", "1".to_string());
        form.set("eps", "1e-8".to_string());
        form.set("n", "50".to_string());
        form.set("max_iter_count", "10000".to_string());
        form.set("dest_file", "y.csv".to_string());

        Self { form }
    }
}

impl ProblemCreator for Fredholm1stProblemCreator {
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
                    Some(&["x", "s"]),
                    &DefaultRuntime::default(),
                    &mut kernel,
                ),
                "right_side" => validate_expr(
                    name,
                    val,
                    Some(&["x"]),
                    &DefaultRuntime::default(),
                    &mut right_side,
                ),
                "from" => validate_from_str::<f64>(name, val, &mut from),
                "to" => validate_from_str::<f64>(name, val, &mut to),
                "eps" => validate_from_str::<f64>(name, val, &mut eps),
                "n" => validate_from_str::<usize>(name, val, &mut n),
                "max_iter_count" => validate_from_str::<usize>(name, val, &mut max_iter_count),
                "dest_file" => Ok(()),
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
        let dest_file = self.form.get("dest_file").ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: dest_file".to_string(),
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
                dest_file: dest_file.cloned().unwrap(),
            }))
        } else {
            Err(errors)
        }
    }

    fn fields(&self) -> super::form::FieldsIter {
        self.form.get_fields()
    }

    fn set_field(&mut self, name: &str, val: String) {
        self.form.set(name, val)
    }
}
