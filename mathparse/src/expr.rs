use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub enum Error {
    UndefinedVariable(String),
    UndefinedFunction(String),
    InvalidArgCount {
        op_name: String,
        got_args: usize,
        expected_args: usize,
    },

    MathError(String),
}

pub trait Runtime {
    fn get_var(&self, name: &str) -> Option<f64>;
    fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, Error>;
    fn has_func(&self, name: &str) -> bool;
}

pub trait Expression {
    fn eval(&self, runtime: &dyn Runtime) -> Result<f64, Error>;
    // fn compile(&self, runtime: &dyn Runtime) -> Result<Box<dyn Expression>, Error>;
    // fn to_number(&self) -> Option<f64>;
    fn query_vars(&self) -> HashSet<&str>;
}

impl Expression for f64 {
    fn eval(&self, _: &dyn Runtime) -> Result<f64, Error> {
        Ok(*self)
    }
    // fn compile(&self, _: &dyn Runtime) -> Result<Box<dyn Expression>, Error> {
    //     Ok(Box::new(*self))
    // }
    // fn to_number(&self) -> Option<f64> {
    //     Some(*self)
    // }
    fn query_vars(&self) -> HashSet<&str> {
        HashSet::new()
    }
}

pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new_expression(name: String) -> Box<dyn Expression + 'static> {
        Box::new(Self { name })
    }
}

impl Expression for Variable {
    fn eval(&self, runtime: &dyn Runtime) -> Result<f64, Error> {
        runtime
            .get_var(&self.name)
            .ok_or_else(|| Error::UndefinedVariable(self.name.clone()))
    }

    // fn compile(&self, runtime:&dyn Runtime) -> Result<Box<dyn Expression + 'a>, Error>
    // where
    //     Self: 'a,
    // {
    //     Ok(variables
    //         .iter()
    //         .find(|(v, _)| v.eq(&self.name))
    //         .map_or_else(
    //             || Variable::new_expression(self.name.clone()),
    //             |(_, val)| Box::new(*val),
    //         ))
    // }
    // fn to_number(&self) -> Option<f64> {
    //     None
    // }

    fn query_vars(&self) -> HashSet<&str> {
        HashSet::from([self.name.as_str()])
    }
}

pub enum BasicOp {
    Plus(Box<dyn Expression>, Box<dyn Expression>),
    Minus(Box<dyn Expression>, Box<dyn Expression>),
    Multiply(Box<dyn Expression>, Box<dyn Expression>),
    Divide(Box<dyn Expression>, Box<dyn Expression>),
    Negate(Box<dyn Expression>),
}

impl Expression for BasicOp {
    fn eval(&self, runtime: &dyn Runtime) -> Result<f64, Error> {
        match self {
            BasicOp::Plus(left, right) => left
                .eval(runtime)
                .and_then(|l| right.eval(runtime).map(|r| l + r)),
            BasicOp::Minus(left, right) => left
                .eval(runtime)
                .and_then(|l| right.eval(runtime).map(|r| l - r)),
            BasicOp::Multiply(left, right) => left
                .eval(runtime)
                .and_then(|l| right.eval(runtime).map(|r| l * r)),
            BasicOp::Divide(left, right) => left
                .eval(runtime)
                .and_then(|l| right.eval(runtime).map(|r| (l, r)))
                .map_or_else(Err, |(l, r)| {
                    if r == 0.0 {
                        Err(Error::MathError("Divide by zero".to_owned()))
                    } else {
                        Ok(l / r)
                    }
                }),
            BasicOp::Negate(r) => r.eval(runtime).map(|res| -res),
        }
    }

    // fn compile<'b>(&self, variables: &[(&str, f64)]) -> Result<Box<dyn Expression + 'b>, Error>
    // where
    //     Self: 'b,
    // {
    //     match self {
    //         BasicOp::Plus(l, r) => {
    //             let l = l.compile(variables)?;
    //             let r = r.compile(variables)?;
    //             Ok(match (l.to_number(), r.to_number()) {
    //                 (None, None) => Box::new(BasicOp::Plus(l, r)),
    //                 (None, Some(r)) => Box::new(BasicOp::Plus(l, Box::new(r))),
    //                 (Some(l), None) => Box::new(BasicOp::Plus(Box::new(l), r)),
    //                 (Some(l), Some(r)) => Box::new(l + r),
    //             })
    //         }
    //         BasicOp::Minus(l, r) => {
    //             let l = l.compile(variables)?;
    //             let r = r.compile(variables)?;
    //             Ok(match (l.to_number(), r.to_number()) {
    //                 (None, None) => Box::new(BasicOp::Minus(l, r)),
    //                 (None, Some(r)) => Box::new(BasicOp::Minus(l, Box::new(r))),
    //                 (Some(l), None) => Box::new(BasicOp::Minus(Box::new(l), r)),
    //                 (Some(l), Some(r)) => Box::new(l - r),
    //             })
    //         }
    //         BasicOp::Multiply(l, r) => {
    //             let l = l.compile(variables)?;
    //             let r = r.compile(variables)?;
    //             Ok(match (l.to_number(), r.to_number()) {
    //                 (None, None) => Box::new(BasicOp::Multiply(l, r)),
    //                 (None, Some(r)) => Box::new(BasicOp::Multiply(l, Box::new(r))),
    //                 (Some(l), None) => Box::new(BasicOp::Multiply(Box::new(l), r)),
    //                 (Some(l), Some(r)) => Box::new(l * r),
    //             })
    //         }
    //         BasicOp::Divide(l, r) => {
    //             let l = l.compile(variables)?;
    //             let r = r.compile(variables)?;

    //             match (l.to_number(), r.to_number()) {
    //                 (None, None) => Ok(Box::new(BasicOp::Divide(l, r))),
    //                 (None, Some(r)) => Ok(Box::new(BasicOp::Divide(l, Box::new(r)))),
    //                 (Some(l), None) => Ok(Box::new(BasicOp::Divide(Box::new(l), r))),
    //                 (Some(l), Some(r)) if r != 0.0 => Ok(Box::new(l / r)),
    //                 _ => Err(Error::MathError("Divide by zero".to_string())),
    //             }
    //         }
    //         BasicOp::Negate(val) => {
    //             let val = val.compile(variables)?;

    //             if let Some(val) = val.to_number() {
    //                 Ok(Box::new(-val))
    //             } else {
    //                 Ok(Box::new(BasicOp::Negate(val)))
    //             }
    //         }
    //     }
    // }

    // fn to_number(&self) -> Option<f64> {
    //     None
    // }

    fn query_vars(&self) -> HashSet<&str> {
        match self {
            BasicOp::Plus(l, r) => l.query_vars().union(&r.query_vars()).copied().collect(),
            BasicOp::Minus(l, r) => l.query_vars().union(&r.query_vars()).copied().collect(),
            BasicOp::Multiply(l, r) => l.query_vars().union(&r.query_vars()).copied().collect(),
            BasicOp::Divide(l, r) => l.query_vars().union(&r.query_vars()).copied().collect(),
            BasicOp::Negate(l) => l.query_vars(),
        }
    }
}

pub struct FunctionExpression {
    args: Vec<Box<dyn Expression>>,
    name: String,
}

impl FunctionExpression {
    pub fn new_expression(args: Vec<Box<dyn Expression>>, name: String) -> Box<dyn Expression> {
        Box::new(Self { args, name })
    }
}

impl Expression for FunctionExpression {
    fn eval(&self, runtime: &dyn Runtime) -> Result<f64, Error> {
        let calculated_args = self
            .args
            .iter()
            .map(|arg| arg.eval(runtime))
            .collect::<Result<Vec<_>, _>>()?;

        runtime.eval_func(&self.name, &calculated_args)
    }

    // fn compile<'b>(&self, variables: &[(&str, f64)]) -> Result<Box<dyn Expression + 'b>, Error>
    // where
    //     Self: 'b,
    // {
    //     let func = self
    //         .language
    //         .borrow()
    //         .find_func(&self.name)
    //         .ok_or_else(|| Error::UndefinedFunction(self.name.clone()))?;

    //     let compiled_args = self
    //         .args
    //         .iter()
    //         .map(|arg| arg.compile(variables))
    //         .collect::<Result<Vec<_>, _>>()?;

    //     if let Some(num_args) = compiled_args
    //         .iter()
    //         .map(|a| a.to_number())
    //         .collect::<Option<Vec<_>>>()
    //     {
    //         Ok(Box::new(func.eval(&num_args)?))
    //     } else {
    //         Ok(Box::new(Self {
    //             language: self.language.clone(),
    //             args: compiled_args,
    //             name: self.name.clone(),
    //         }))
    //         // Ok(FunctionExpression::new_expression(
    //         //     self.language.borrow(),
    //         //     compiled_args,
    //         //     self.name.clone(),
    //         // ))
    //     }
    // }

    // fn to_number(&self) -> Option<f64> {
    //     None
    // }

    fn query_vars(&self) -> HashSet<&str> {
        self.args
            .iter()
            .map(|a| a.query_vars())
            .fold(HashSet::new(), |acc, vars| {
                acc.union(&vars).copied().collect()
            })
    }
}

#[derive(Default, Debug)]
pub struct DefaultRuntime {
    vars: HashMap<String, f64>,
}

impl DefaultRuntime {
    pub fn new(vars: &[(&str, f64)]) -> Self {
        Self {
            vars: HashMap::from_iter(vars.iter().map(|(n, v)| (n.to_string(), *v))),
        }
    }
}

impl Runtime for DefaultRuntime {
    fn get_var(&self, name: &str) -> Option<f64> {
        self.vars.get(name).copied()
    }

    fn has_func(&self, name: &str) -> bool {
        ["sin", "cos", "pow", "exp", "sqrt", "ln"]
            .into_iter()
            .any(|v| v.eq(name))
    }

    fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, Error> {
        match name {
            "sin" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "sin".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else {
                    Ok(args[0].sin())
                }
            }
            "cos" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "cos".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else {
                    Ok(args[0].cos())
                }
            }
            "pow" => {
                if args.len() != 2 {
                    Err(Error::InvalidArgCount {
                        op_name: "pow".to_string(),
                        got_args: args.len(),
                        expected_args: 2,
                    })
                } else {
                    Ok(args[0].powf(args[1]))
                }
            }
            "sqrt" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "sqrt".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else if args[0] < 0.0 {
                    Err(Error::MathError("Sqrt of negative".to_owned()))
                } else {
                    Ok(args[0].sqrt())
                }
            }
            "exp" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "exp".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else {
                    Ok(args[0].exp())
                }
            }
            "ln" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "ln".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else if args[0] < 0.0 {
                    Err(Error::MathError("Log of negative".to_owned()))
                } else {
                    Ok(args[0].ln())
                }
            }
            _ => Err(Error::UndefinedFunction(name.to_string())),
        }
    }
}
