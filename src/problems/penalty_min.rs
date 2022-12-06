use std::collections::HashMap;

use crate::{
    function::function::Function,
    mathparse::{DefaultRuntime, Error, Expression},
    min_find::penalty_min::penalty_min,
};

use super::{
    form::Form,
    graph::{Graph, Path},
    validate_expr, validate_from_str, Problem, ProblemCreator, Solution, SolutionParagraph,
    ValidationError,
};

struct PenaltyMinProblem {
    f: Box<dyn Expression>,
    constraints: Vec<Box<dyn Expression>>,
    from: f64,
    to: f64,
    start_eps: f64,
    min_step: f64,
    max_iter_count: usize,
}

impl Problem for PenaltyMinProblem {
    fn solve(&self) -> Solution {
        let c = self
            .constraints
            .iter()
            .map(|f| |x| f.eval(&DefaultRuntime::new(&[("x", x)])))
            .collect::<Vec<_>>();

        let f = |x| self.f.eval(&DefaultRuntime::new(&[("x", x)]));
        let res = penalty_min(
            &f,
            &c.iter()
                .map(|f| f as &dyn Function<Error = Error>)
                .collect::<Vec<_>>(),
            self.from,
            self.to,
            self.start_eps,
            self.min_step,
            self.max_iter_count,
        );
        match res {
            Ok(res) => {
                let graphs = c
                    .iter()
                    .map(|c| c.sample(self.from, self.to, 20))
                    .map(|pts| {
                        pts.map(|p| Path {
                            pts: p,
                            kind: super::graph::PathKind::Line,
                            color: (0.0, 1.0, 0.0),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>();
                let graphs = graphs
                    .and_then(|mut g| {
                        f.sample(self.from, self.to, 20).map(|f_pts| {
                            g.push(Path {
                                pts: f_pts,
                                kind: super::graph::PathKind::Line,
                                color: (1.0, 0.0, 0.0),
                            });
                            g.push(Path {
                                pts: vec![(res.x, res.y)],
                                kind: super::graph::PathKind::Dot,
                                color: (0.0, 0.0, 1.0),
                            });
                            g
                        })
                    })
                    .map_err(|e| format!("{:?}", e));

                let graph = graphs.and_then(|paths| {
                    Graph::new(paths).ok_or_else(|| "Could not create graph".to_string())
                });

                let mut expl = vec![
                    SolutionParagraph::Text(format!("Min at ({:.4}, {:.4})", res.x, res.y)),
                    SolutionParagraph::Latex(format!(
                        "f(x)={{{}}}",
                        self.f
                            .to_latex(&DefaultRuntime::default())
                            .unwrap_or_else(|_| String::new())
                    )),
                ];

                for (i, c) in self.constraints.iter().enumerate() {
                    expl.push(SolutionParagraph::Latex(format!(
                        "g_{i}={{{}}}<0",
                        c.to_latex(&DefaultRuntime::default())
                            .unwrap_or_else(|_| String::new())
                    )))
                }

                expl.push(match graph {
                    Ok(g) => SolutionParagraph::Graph(g),
                    Err(e) => SolutionParagraph::RuntimeError(e),
                });

                Solution { explanation: expl }
            }
            Err(e) => Solution {
                explanation: vec![SolutionParagraph::RuntimeError(format!("{:?}", e))],
            },
        }
    }
}

pub struct PenaltyMinProblemCreator {
    form: Form,
    constraint_count: usize,
}

impl Default for PenaltyMinProblemCreator {
    fn default() -> Self {
        let mut form = Form::new(vec![
            "f".to_string(),
            "from".to_string(),
            "to".to_string(),
            "start_eps".to_string(),
            "min_step".to_string(),
            "max_iter_count".to_string(),
            "constraint1".to_string(),
            "constraint2".to_string(),
        ]);

        form.set("f", "-0.8pow(x,4)-1.2pow(x,3)+pow(x,2)+x".to_string());
        form.set("from", "-2".to_string());
        form.set("to", "1".to_string());
        form.set("start_eps", "0.001".to_string());
        form.set("min_step", "0.001".to_string());
        form.set("max_iter_count", "1000".to_string());
        form.set("constraint1", "-x-1".to_string());

        Self {
            form,
            constraint_count: 2,
        }
    }
}

impl ProblemCreator for PenaltyMinProblemCreator {
    fn try_create(&self) -> Result<Box<dyn Problem>, Vec<super::ValidationError>> {
        let mut f = None;
        let mut from = None;
        let mut to = None;
        let mut start_eps = None;
        let mut min_step = None;
        let mut max_iter_count = None;

        let mut constraints: HashMap<usize, Option<Box<dyn Expression>>> = HashMap::new();
        let mut errors = vec![];

        for (name, val) in self.fields() {
            let res = match name {
                "f" => validate_expr("f", val, Some(&["x"]), &DefaultRuntime::default(), &mut f),
                "from" => validate_from_str("from", val, &mut from),
                "to" => validate_from_str("to", val, &mut to),
                "start_eps" => validate_from_str("start_eps", val, &mut start_eps),
                "min_step" => validate_from_str("min_step", val, &mut min_step),
                "max_iter_count" => validate_from_str("max_iter_count", val, &mut max_iter_count),
                _ => {
                    if let Some(index) = name.strip_prefix("constraint") {
                        index
                            .parse::<usize>()
                            .map_err(|e| {
                                ValidationError(format!(
                                    "{name} - invalid name, should end with a number ({:?})",
                                    e
                                ))
                            })
                            .and_then(|i| {
                                if val.is_empty() {
                                    constraints.insert(i, None);
                                    Ok(())
                                } else {
                                    validate_expr(
                                        name,
                                        val,
                                        Some(&["x"]),
                                        &DefaultRuntime::default(),
                                        constraints.entry(i).or_insert(None),
                                    )
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

        let f =
            f.ok_or_else(|| errors.push(ValidationError("field was not supplied: f".to_string())));
        let from = from.ok_or_else(|| {
            errors.push(ValidationError("field was not supplied: from".to_string()))
        });
        let to = to
            .ok_or_else(|| errors.push(ValidationError("field was not supplied: to".to_string())));
        let start_eps = start_eps.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: start_eps".to_string(),
            ))
        });
        let min_step = min_step.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: min_step".to_string(),
            ))
        });
        let max_iter_count = max_iter_count.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: max_iter_count".to_string(),
            ))
        });

        if errors.is_empty() {
            Ok(Box::new(PenaltyMinProblem {
                f: f.unwrap(),
                from: from.unwrap(),
                to: to.unwrap(),
                start_eps: start_eps.unwrap(),
                min_step: min_step.unwrap(),
                max_iter_count: max_iter_count.unwrap(),
                constraints: constraints.into_values().flatten().collect(),
            }))
        } else {
            Err(errors)
        }
    }

    fn fields(&self) -> super::form::FieldsIter {
        self.form.get_fields()
    }

    fn set_field(&mut self, name: &str, val: String) {
        if let Some(index) = name.strip_prefix("constraint") {
            if let Ok(i) = index.parse::<usize>() {
                if i == self.constraint_count {
                    self.constraint_count += 1;
                    self.form
                        .add_field(format!("constraint{}", self.constraint_count));
                }

                self.form.set(name, val);
            }
        } else {
            self.form.set(name, val);
        }
    }
}
