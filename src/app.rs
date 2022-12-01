use std::collections::LinkedList;

use crate::problems::{
    area_calc::AreaCalcProblemCreator, fredholm_1st::Fredholm1stProblemCreator,
    wolterra_2nd::Wolterra2ndProblemCreator, Problem, ProblemCreator, Solution, ValidationError,
};

pub struct AppState {
    problem_creators: Vec<Box<dyn ProblemCreator>>,
    cur_problem_creator: usize,
    prepared_problem: Option<Box<dyn Problem>>,
    validation_errors: Vec<ValidationError>,
    solutions: LinkedList<Solution>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            problem_creators: vec![
                Box::new(Fredholm1stProblemCreator::default()),
                Box::new(AreaCalcProblemCreator::default()),
                Box::new(Wolterra2ndProblemCreator::default()),
            ],
            cur_problem_creator: 0,
            prepared_problem: None,
            validation_errors: Vec::new(),
            solutions: LinkedList::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProblemName {
    FredholmFirst,
    AreaCalc,
    WolterraSecond,
}

impl ProblemName {
    fn to_index(&self) -> usize {
        match self {
            ProblemName::FredholmFirst => 0,
            ProblemName::AreaCalc => 1,
            ProblemName::WolterraSecond => 2,
        }
    }
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(ProblemName::FredholmFirst),
            1 => Some(ProblemName::AreaCalc),
            2 => Some(ProblemName::WolterraSecond),
            _ => None,
        }
    }
}

impl ToString for ProblemName {
    fn to_string(&self) -> String {
        match self {
            ProblemName::FredholmFirst => "Fredholm first kind".to_string(),
            ProblemName::AreaCalc => "Area".to_string(),
            ProblemName::WolterraSecond => "Wolterra second kind".to_string(),
        }
    }
}

impl AppState {
    fn cur(&self) -> &dyn ProblemCreator {
        self.problem_creators[self.cur_problem_creator].as_ref()
    }
    fn mut_cur(&mut self) -> &mut dyn ProblemCreator {
        self.problem_creators[self.cur_problem_creator].as_mut()
    }

    pub fn get_problems(&self) -> Vec<ProblemName> {
        vec![
            ProblemName::FredholmFirst,
            ProblemName::AreaCalc,
            ProblemName::WolterraSecond,
        ]
    }
    pub fn set_problem(&mut self, name: ProblemName) {
        self.cur_problem_creator = name.to_index();
    }
    pub fn get_cur_problem(&self) -> Option<ProblemName> {
        ProblemName::from_index(self.cur_problem_creator)
    }

    pub fn fields(&self) -> impl Iterator<Item = (&str, &str)> {
        self.cur().form().get_fields()
    }
    pub fn set_field(&mut self, name: &str, val: String) {
        self.mut_cur().form_mut().set(name, val);
    }
    pub fn get_validation_errors(&self) -> &[ValidationError] {
        &self.validation_errors
    }

    pub fn validate(&mut self) {
        self.validation_errors.clear();
        self.prepared_problem = match self.cur().try_create() {
            Ok(p) => Some(p),
            Err(e) => {
                self.validation_errors = e;
                None
            }
        }
    }
    pub fn solve(&mut self) {
        match &self.prepared_problem {
            Some(p) => {
                let res = p.solve();
                dbg!(&res);
                self.solutions.push_back(res);
            }
            None => {}
        }
    }

    pub fn get_solutions(&self) -> impl Iterator<Item = &Solution> {
        self.solutions.iter()
    }
    pub fn rem_solution(&mut self, index: usize) {
        let mut split_list = self.solutions.split_off(index);
        split_list.pop_front();
        self.solutions.append(&mut split_list);
    }
}
