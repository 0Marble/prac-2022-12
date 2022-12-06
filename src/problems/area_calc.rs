use crate::{
    area_calc::calc_area,
    function::function::Function,
    mathparse::{DefaultRuntime, Expression},
};

use super::{
    form::Form,
    graph::{Graph, Path},
    validate_expr, validate_from_str, Problem, ProblemCreator, Solution, SolutionParagraph,
    ValidationError,
};

struct AreaCalcProblem {
    f1: Box<dyn Expression>,
    f2: Box<dyn Expression>,
    f3: Box<dyn Expression>,
    x12: [f64; 2],
    x13: [f64; 2],
    x23: [f64; 2],
    eps: f64,
    max_iter_count: usize,
}

impl Problem for AreaCalcProblem {
    fn solve(&self) -> super::Solution {
        let f1 = |x| self.f1.eval(&DefaultRuntime::new(&[("x", x)]));
        let f2 = |x| self.f2.eval(&DefaultRuntime::new(&[("x", x)]));
        let f3 = |x| self.f3.eval(&DefaultRuntime::new(&[("x", x)]));

        let res = calc_area(
            &f1,
            &f2,
            &f3,
            self.x12,
            self.x13,
            self.x23,
            0.001,
            self.eps,
            self.max_iter_count,
        );

        match res {
            Ok(area) => {
                let mut expl = vec![
                    SolutionParagraph::Text(format!(
                        "Area = {:.4}, x12 = {:.4}, x13 = {:.4}, x23 = {:.4}",
                        area.area, area.x12, area.x13, area.x23
                    )),
                    SolutionParagraph::Latex(format!(
                        "f_1(x)={{{}}}",
                        self.f1
                            .to_latex(&DefaultRuntime::default())
                            .unwrap_or_else(|_| String::new())
                    )),
                    SolutionParagraph::Latex(format!(
                        "f_2(x)={{{}}}",
                        self.f2
                            .to_latex(&DefaultRuntime::default())
                            .unwrap_or_else(|_| String::new())
                    )),
                    SolutionParagraph::Latex(format!(
                        "f_3(x)={{{}}}",
                        self.f3
                            .to_latex(&DefaultRuntime::default())
                            .unwrap_or_else(|_| String::new())
                    )),
                ];

                let p1 = f1.sample(
                    f64::min(self.x12[0], self.x13[0]),
                    f64::max(self.x12[1], self.x13[1]),
                    50,
                );
                let p3 = f3.sample(
                    f64::min(self.x23[0], self.x13[0]),
                    f64::max(self.x23[1], self.x13[1]),
                    50,
                );
                let p2 = f2.sample(
                    f64::min(self.x23[0], self.x12[0]),
                    f64::max(self.x23[1], self.x12[1]),
                    50,
                );
                if let Err(e) = &p1 {
                    expl.push(SolutionParagraph::RuntimeError(format!("{:?}", e)));
                }
                if let Err(e) = &p2 {
                    expl.push(SolutionParagraph::RuntimeError(format!("{:?}", e)));
                }
                if let Err(e) = &p3 {
                    expl.push(SolutionParagraph::RuntimeError(format!("{:?}", e)));
                }
                let seg_1 = area.f1.sample(area.x12, area.x13, 20);
                let seg_3 = area.f3.sample(area.x13, area.x23, 20);
                let seg_2 = area.f2.sample(area.x23, area.x12, 20);
                if let Err(e) = &seg_1 {
                    expl.push(SolutionParagraph::RuntimeError(format!("{:?}", e)));
                }
                if let Err(e) = &seg_2 {
                    expl.push(SolutionParagraph::RuntimeError(format!("{:?}", e)));
                }
                if let Err(e) = &seg_3 {
                    expl.push(SolutionParagraph::RuntimeError(format!("{:?}", e)));
                }

                if let (Ok(p1), Ok(p2), Ok(p3), Ok(mut seg_1), Ok(mut seg_2), Ok(mut seg_3)) =
                    (p1, p2, p3, seg_1, seg_2, seg_3)
                {
                    let mut a = vec![];
                    a.append(&mut seg_1);
                    a.append(&mut seg_3);
                    a.append(&mut seg_2);

                    let g = Graph::new(vec![
                        Path {
                            pts: a,
                            kind: super::graph::PathKind::Filled,
                            color: (0.5, 0.5, 0.5),
                        },
                        Path {
                            pts: p1,
                            kind: super::graph::PathKind::Line,
                            color: (1.0, 0.0, 0.0),
                        },
                        Path {
                            pts: p2,
                            kind: super::graph::PathKind::Line,
                            color: (0.0, 1.0, 0.0),
                        },
                        Path {
                            pts: p3,
                            kind: super::graph::PathKind::Line,
                            color: (0.0, 0.0, 1.0),
                        },
                    ]);

                    match g {
                        Some(g) => expl.push(SolutionParagraph::Graph(g)),
                        None => expl.push(SolutionParagraph::RuntimeError(
                            "Could not draw a graph".to_string(),
                        )),
                    }
                }

                Solution { explanation: expl }
            }
            Err(e) => Solution {
                explanation: vec![SolutionParagraph::RuntimeError(format!("{:?}", e))],
            },
        }
    }
}

pub struct AreaCalcProblemCreator {
    form: Form,
}

impl Default for AreaCalcProblemCreator {
    fn default() -> Self {
        let mut form = Form::new(vec![
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
        ]);

        form.set("f1", "exp(x)+2".to_string());
        form.set("f2", "-2x+8".to_string());
        form.set("f3", "-5/x".to_string());
        form.set("x12_from", "0".to_string());
        form.set("x12_to", "2".to_string());
        form.set("x13_from", "-4".to_string());
        form.set("x13_to", "-1".to_string());
        form.set("x23_from", "-2".to_string());
        form.set("x23_to", "-0.3".to_string());
        form.set("eps", "0.001".to_string());
        form.set("max_iter_count", "1000".to_string());

        Self { form }
    }
}

impl ProblemCreator for AreaCalcProblemCreator {
    fn try_create(&self) -> Result<Box<dyn Problem>, Vec<ValidationError>> {
        let mut f1 = None;
        let mut f2 = None;
        let mut f3 = None;
        let mut x12_from = None;
        let mut x12_to = None;
        let mut x13_from = None;
        let mut x13_to = None;
        let mut x23_from = None;
        let mut x23_to = None;
        let mut eps = None;
        let mut max_iter_count = None;

        let mut errors = vec![];

        for (name, val) in self.form.get_fields() {
            let res = match name {
                "f1" => validate_expr("f1", val, Some(&["x"]), &DefaultRuntime::default(), &mut f1),
                "f2" => validate_expr("f2", val, Some(&["x"]), &DefaultRuntime::default(), &mut f2),
                "f3" => validate_expr("f3", val, Some(&["x"]), &DefaultRuntime::default(), &mut f3),
                "x12_from" => validate_from_str::<f64>("x12_from", val, &mut x12_from),
                "x12_to" => validate_from_str::<f64>("x12_to", val, &mut x12_to),
                "x13_from" => validate_from_str::<f64>("x13_from", val, &mut x13_from),
                "x13_to" => validate_from_str::<f64>("x13_to", val, &mut x13_to),
                "x23_from" => validate_from_str::<f64>("x23_from", val, &mut x23_from),
                "x23_to" => validate_from_str::<f64>("x23_to", val, &mut x23_to),
                "eps" => validate_from_str::<f64>("eps", val, &mut eps),
                "max_iter_count" => {
                    validate_from_str::<usize>("max_iter_count", val, &mut max_iter_count)
                }
                _ => Err(ValidationError(format!(
                    "{name} - no such field (probably a devs error)"
                ))),
            };

            match res {
                Ok(_) => {}
                Err(e) => errors.push(e),
            }
        }

        let f1 = f1
            .ok_or_else(|| errors.push(ValidationError("field was not supplied: f1".to_string())));
        let f2 = f2
            .ok_or_else(|| errors.push(ValidationError("field was not supplied: f2".to_string())));
        let f3 = f3
            .ok_or_else(|| errors.push(ValidationError("field was not supplied: f3".to_string())));
        let x12_from = x12_from.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: x12_from".to_string(),
            ))
        });
        let x12_to = x12_to.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: x12_to".to_string(),
            ))
        });
        let x13_from = x13_from.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: x13_from".to_string(),
            ))
        });
        let x13_to = x13_to.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: x13_to".to_string(),
            ))
        });
        let x23_from = x23_from.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: x23_from".to_string(),
            ))
        });
        let x23_to = x23_to.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: x23_to".to_string(),
            ))
        });
        let eps = eps
            .ok_or_else(|| errors.push(ValidationError("field was not supplied: eps".to_string())));
        let max_iter_count = max_iter_count.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied: max_iter_count".to_string(),
            ))
        });

        if errors.is_empty() {
            Ok(Box::new(AreaCalcProblem {
                f1: f1.unwrap(),
                f2: f2.unwrap(),
                f3: f3.unwrap(),
                x12: [x12_from.unwrap(), x12_to.unwrap()],
                x13: [x13_from.unwrap(), x13_to.unwrap()],
                x23: [x23_from.unwrap(), x23_to.unwrap()],
                eps: eps.unwrap(),
                max_iter_count: max_iter_count.unwrap(),
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
