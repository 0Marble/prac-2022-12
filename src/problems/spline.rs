use std::{fs::File, io::Write, path::Path as FilePath};

use crate::{
    common::{function::Function, table_function::TableFunction},
    spline::Spline,
};

use super::{
    form::Form,
    graph::{Graph, Path},
    Problem, ProblemCreator, Solution, SolutionParagraph, ValidationError,
};

struct SplieProblem {
    src_file: String,
    dest_file: String,
}

impl Problem for SplieProblem {
    fn solve(&self) -> super::Solution {
        let func = TableFunction::from_file(FilePath::new(&self.src_file));
        let dest_file = File::create(&self.dest_file);
        let res = func
            .map_err(|e| format!("{:?}", e))
            .and_then(|func| {
                dest_file
                    .map_err(|e| format!("{:?}", e))
                    .map(|dest| (func, dest))
            })
            .and_then(|(func, mut dest)| {
                let spline = Spline::new(func.to_table());
                spline
                    .write_coefs()
                    .map_err(|e| format!("{:?}", e))
                    .and_then(|coefs| write!(dest, "{}", coefs).map_err(|e| format!("{:?}", e)))
                    .map(|()| (func.to_table(), spline, func.min_x(), func.max_x()))
            })
            .and_then(|(table, spline, from, to)| {
                if let (Some(min), Some(max)) = (from, to) {
                    spline
                        .sample(min, max, 50)
                        .map_err(|e| format!("{:?}", e))
                        .map(|spline| (table, spline))
                } else {
                    Err("No points given".to_string())
                }
            })
            .and_then(|(table_pts, spline_pts)| {
                Graph::new(vec![
                    Path {
                        pts: spline_pts,
                        kind: super::graph::PathKind::Line,
                        color: (1.0, 0.0, 0.0),
                    },
                    Path {
                        pts: table_pts,
                        kind: super::graph::PathKind::Dot,
                        color: (0.0, 0.0, 1.0),
                    },
                ])
                .ok_or_else(|| "Could not create graph".to_string())
            });

        match res {
            Ok(res) => Solution {
                explanation: vec![
                    SolutionParagraph::Text(format!(
                        "{} saved in {}",
                        self.src_file, self.dest_file
                    )),
                    SolutionParagraph::Graph(res),
                ],
            },
            Err(e) => Solution {
                explanation: vec![SolutionParagraph::RuntimeError(e)],
            },
        }
    }
}

pub struct SplineProblemCreator {
    form: Form,
}

impl Default for SplineProblemCreator {
    fn default() -> Self {
        let mut form = Form::new(vec!["src_file".to_string(), "dest_file".to_string()]);
        form.set("src_file", "pts.csv".to_string());
        form.set("dest_file", "spline.csv".to_string());

        Self { form }
    }
}

impl ProblemCreator for SplineProblemCreator {
    fn fields(&self) -> super::form::FieldsIter {
        self.form.get_fields()
    }

    fn set_field(&mut self, name: &str, val: String) {
        self.form.set(name, val)
    }

    fn try_create(&self) -> Result<Box<dyn Problem>, Vec<super::ValidationError>> {
        let mut src_file = None;
        let mut dest_file = None;

        let mut errors = vec![];
        for (name, val) in self.form.get_fields() {
            match name {
                "src_file" => src_file = Some(val),
                "dest_file" => dest_file = Some(val),
                _ => errors.push(ValidationError(format!(
                    "{name} - no such field (probably a devs error)"
                ))),
            }
        }

        let src_file = src_file.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied - src_file".to_string(),
            ))
        });
        let dest_file = dest_file.ok_or_else(|| {
            errors.push(ValidationError(
                "field was not supplied - dest_file".to_string(),
            ))
        });

        if errors.is_empty() {
            Ok(Box::new(SplieProblem {
                src_file: src_file.unwrap().to_string(),
                dest_file: dest_file.unwrap().to_string(),
            }))
        } else {
            Err(errors)
        }
    }
}
