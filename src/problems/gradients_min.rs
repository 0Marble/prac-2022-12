use std::collections::HashMap;

use crate::{
    functions::function::FunctionNd,
    mathparse::{parse, DefaultRuntime, Error, Expression},
    min_find::gradients_min::gradients_min,
};

use super::{
    form::Form,
    graph::{Graph, Path},
    validate_expr, validate_from_str, Problem, ProblemCreator, Solution, SolutionParagraph,
    ValidationError,
};

struct GradientsMinProblem {
    ordered_vars: Vec<String>,
    f: Box<dyn Expression>,
    grad: Vec<Box<dyn Expression>>,
    x0: Vec<f64>,
    eps: f64,
    max_iter_count: usize,
}

impl Problem for GradientsMinProblem {
    fn solve(&self) -> super::Solution {
        let f = |x: &[f64]| {
            self.f.eval(&DefaultRuntime::new(
                &self
                    .ordered_vars
                    .iter()
                    .enumerate()
                    .map(|(i, name)| (name.as_str(), x[i]))
                    .collect::<Vec<_>>(),
            ))
        };

        let grad = self
            .grad
            .iter()
            .map(|f| {
                |x: &[f64]| {
                    f.eval(&DefaultRuntime::new(
                        &self
                            .ordered_vars
                            .iter()
                            .enumerate()
                            .map(|(i, name)| (name.as_str(), x[i]))
                            .collect::<Vec<_>>(),
                    ))
                }
            })
            .collect::<Vec<_>>();

        let res = gradients_min(
            &f,
            &grad
                .iter()
                .map(|f| f as &dyn FunctionNd<Error = Error>)
                .collect::<Vec<_>>(),
            &self.x0,
            self.eps,
            self.max_iter_count,
        );

        match res {
            Ok(res) => {
                let mut paragraphs = vec![
                    SolutionParagraph::Text(format!("Min at ({:?}, {:.4})", res.x, res.y)),
                    SolutionParagraph::Latex(format!(
                        "f(x)={{{}}}",
                        self.f
                            .to_latex(&DefaultRuntime::default())
                            .unwrap_or_else(|_| String::new())
                    )),
                ];

                for (df, var) in self.grad.iter().zip(self.ordered_vars.iter()) {
                    paragraphs.push(SolutionParagraph::Latex(format!(
                        "\\frac{{\\partial f}}{{\\partial {var}}}={{{}}}",
                        df.to_latex(&DefaultRuntime::default())
                            .unwrap_or_else(|_| String::new())
                    )));
                }

                if self.x0.len() == 1 {
                    let x = res.x[0];
                    let pts = f.sample(&[x - 2.0], &[x + 2.0], &[20]);
                    match pts {
                        Ok(pts) => match Graph::new(vec![
                            Path {
                                pts: pts.iter().map(|p| (p[0], p[1])).collect(),
                                kind: super::graph::PathKind::Line,
                                color: (1.0, 0.0, 0.0),
                            },
                            Path {
                                pts: vec![(res.x[0], res.y)],
                                kind: super::graph::PathKind::Dot,
                                color: (0.0, 0.0, 1.0),
                            },
                        ]) {
                            Some(g) => paragraphs.push(SolutionParagraph::Graph(g)),
                            None => paragraphs.push(SolutionParagraph::RuntimeError(
                                "Could not create graph".to_string(),
                            )),
                        },
                        Err(e) => {
                            paragraphs.push(SolutionParagraph::RuntimeError(format!("{:?}", e)))
                        }
                    }
                }

                Solution {
                    explanation: paragraphs,
                }
            }
            Err(e) => Solution {
                explanation: vec![SolutionParagraph::RuntimeError(format!("{:?}", e))],
            },
        }
    }
}

pub struct GradientsMinProblemCreator {
    form: Form,
    ordered_vars: Vec<String>,
}

impl Default for GradientsMinProblemCreator {
    fn default() -> Self {
        let mut form = Form::new(vec![
            "f".to_string(),
            "eps".to_string(),
            "max_iter_count".to_string(),
            "df/dx".to_string(),
            "df/dy".to_string(),
            "x0".to_string(),
            "y0".to_string(),
        ]);

        form.set("f", "10pow(y-x*x,2)+pow(1-x,2)".to_string());
        form.set("eps", "0.00001".to_string());
        form.set("max_iter_count", "10000".to_string());
        form.set("df/dx", "-40x*y+40pow(x,3)+2x-2".to_string());
        form.set("df/dy", "20y-20*x*x".to_string());
        form.set("x0", "3".to_string());
        form.set("y0", "3".to_string());

        Self {
            form,
            ordered_vars: vec!["x".to_string(), "y".to_string()],
        }
    }
}

impl ProblemCreator for GradientsMinProblemCreator {
    fn fields(&self) -> super::form::FieldsIter {
        self.form.get_fields()
    }

    fn set_field(&mut self, name: &str, val: String) {
        if name == "f" {
            if let Some(expr) = parse(&val, &DefaultRuntime::default()) {
                let new_vars =
                    Vec::from_iter(expr.query_vars().iter().map(|name| name.to_string()));

                let mut new_form = Form::new(vec![
                    "f".to_string(),
                    "eps".to_string(),
                    "max_iter_count".to_string(),
                ]);

                if let Some(val) = self.form.get("f") {
                    new_form.set("f", val.clone())
                }
                if let Some(val) = self.form.get("eps") {
                    new_form.set("eps", val.clone())
                }
                if let Some(val) = self.form.get("max_iter_count") {
                    new_form.set("max_iter_count", val.clone())
                }

                for name in &new_vars {
                    new_form.add_field(format!("{name}0"));
                }

                for name in &new_vars {
                    new_form.add_field(format!("df/d{name}"));
                }

                self.form = new_form;
                self.ordered_vars = new_vars;
            }
        }
        self.form.set(name, val);
    }

    fn try_create(&self) -> Result<Box<dyn Problem>, Vec<super::ValidationError>> {
        let mut f = None;
        let mut eps = None;
        let mut max_iter_count = None;
        let mut x0 = HashMap::new();
        let mut grad = HashMap::new();

        let mut errors = vec![];
        let allowed_vars = self
            .ordered_vars
            .iter()
            .map(|name| name.as_str())
            .collect::<Vec<_>>();

        for (name, val) in self.fields() {
            let res =
                match name {
                    "f" => validate_expr(
                        name,
                        val,
                        Some(&allowed_vars),
                        &DefaultRuntime::default(),
                        &mut f,
                    ),
                    "eps" => validate_from_str::<f64>(name, val, &mut eps),
                    "max_iter_count" => validate_from_str::<usize>(name, val, &mut max_iter_count),
                    _ => {
                        if let Some(var_name) = name.strip_suffix('0') {
                            let mut var_value = None;
                            validate_from_str::<f64>(name, val, &mut var_value).and_then(|_| {
                                match self.ordered_vars.iter().find(|name| name.eq(&var_name)) {
                                    Some(_) => {
                                        x0.insert(var_name.to_string(), var_value.unwrap());
                                        Ok(())
                                    }
                                    None => Err(ValidationError(format!(
                                        "{name} - no such field (probably a devs error) "
                                    ))),
                                }
                            })
                        } else if let Some(var_name) = name.strip_prefix("df/d") {
                            let mut var_value = None;
                            validate_expr(
                                name,
                                val,
                                Some(&allowed_vars),
                                &DefaultRuntime::default(),
                                &mut var_value,
                            )
                            .and_then(|_| {
                                match self.ordered_vars.iter().find(|name| name.eq(&var_name)) {
                                    Some(_) => {
                                        grad.insert(var_name.to_string(), var_value.unwrap());
                                        Ok(())
                                    }
                                    None => Err(ValidationError(format!(
                                        "{name} - no such field (probably a devs error) "
                                    ))),
                                }
                            })
                        } else {
                            Err(ValidationError(format!(
                                "{name} - no such field (probably a devs error)"
                            )))
                        }
                    }
                };

            match res {
                Ok(_) => {}
                Err(e) => errors.push(e),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let f =
            f.ok_or_else(|| errors.push(ValidationError("field f was not supplied".to_string())));
        let eps = eps
            .ok_or_else(|| errors.push(ValidationError("field eps was not supplied".to_string())));
        let max_iter_count = max_iter_count.ok_or_else(|| {
            errors.push(ValidationError(
                "field max_iter_count was not supplied".to_string(),
            ))
        });

        if !grad
            .keys()
            .all(|name| allowed_vars.iter().any(|allowed_name| allowed_name == name))
            || grad.len() != allowed_vars.len()
        {
            errors.push(ValidationError(
                "Not all derivatives were supplied".to_string(),
            ));
        }

        if !x0
            .keys()
            .all(|name| allowed_vars.iter().any(|allowed_name| allowed_name == name))
            || x0.len() != allowed_vars.len()
        {
            errors.push(ValidationError(
                "Not all x0 coordinates were supplied".to_string(),
            ));
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Box::new(GradientsMinProblem {
                ordered_vars: self.ordered_vars.clone(),
                f: f.unwrap(),
                grad: grad.into_values().collect(),
                x0: x0.values().cloned().collect(),
                eps: eps.unwrap(),
                max_iter_count: max_iter_count.unwrap(),
            }))
        }
    }
}
