use crate::{
    common::function::Function,
    mathparse::{DefaultRuntime, Error, Expression},
    min_find::penalty_min::penalty_min,
};

use super::{form::Form, Problem, ProblemCreator, Solution};

pub struct PenaltyMinProblem {
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

        let res = penalty_min(
            &|x| self.f.eval(&DefaultRuntime::new(&[("x", x)])),
            &c.iter()
                .map(|f| f as &dyn Function<Error = Error>)
                .collect::<Vec<_>>(),
            self.from,
            self.to,
            self.start_eps,
            self.min_step,
            self.max_iter_count,
        );

        todo!()
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
            "constraint3".to_string(),
        ]);

        form.set("f", "-3pow(x,4)-pow(x,3)+4pow(x,2)+2x-1".to_string());
        form.set("from", "-10".to_string());
        form.set("to", "10".to_string());
        form.set("start_eps", "0.001".to_string());
        form.set("min_step", "0.001".to_string());
        form.set("max_iter_count", "1000".to_string());
        form.set("constraint1", "pow(x,2)-1".to_string());
        form.set("constraint2", "-sin(10x)-0.5".to_string());

        Self {
            form,
            constraint_count: 2,
        }
    }
}

impl ProblemCreator for PenaltyMinProblemCreator {
    fn form(&self) -> &Form {
        todo!()
    }

    fn form_mut(&mut self) -> &mut Form {
        todo!()
    }

    fn try_create(&self) -> Result<Box<dyn Problem>, Vec<super::ValidationError>> {
        todo!()
    }
}
