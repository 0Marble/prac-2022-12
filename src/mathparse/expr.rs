use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UndefinedVariable(String),
    UndefinedFunction(String),
    InvalidArgCount {
        op_name: String,
        got_args: usize,
        expected_args: usize,
    },

    Math(String),
}

pub trait Runtime {
    fn get_var(&self, name: &str) -> Option<f64>;
    fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, Error>;
    fn has_func(&self, name: &str) -> bool;
    fn to_latex(&self, name: &str, args: &[String]) -> Result<String, Error>;
}

pub trait Expression: Debug {
    fn eval(&self, runtime: &dyn Runtime) -> Result<f64, Error>;
    fn query_vars(&self) -> HashSet<&str>;
    fn to_latex(&self, runtime: &dyn Runtime) -> Result<String, Error>;
}

impl Expression for f64 {
    fn eval(&self, _: &dyn Runtime) -> Result<f64, Error> {
        Ok(*self)
    }

    fn query_vars(&self) -> HashSet<&str> {
        HashSet::new()
    }

    fn to_latex(&self, _: &dyn Runtime) -> Result<String, Error> {
        Ok(self.to_string())
    }
}

#[derive(Debug, Clone)]
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

    fn query_vars(&self) -> HashSet<&str> {
        HashSet::from([self.name.as_str()])
    }

    fn to_latex(&self, _: &dyn Runtime) -> Result<String, Error> {
        Ok(self.name.clone())
    }
}

#[derive(Debug)]
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
                        Err(Error::Math("Divide by zero".to_owned()))
                    } else {
                        Ok(l / r)
                    }
                }),
            BasicOp::Negate(r) => r.eval(runtime).map(|res| -res),
        }
    }

    fn query_vars(&self) -> HashSet<&str> {
        match self {
            BasicOp::Plus(l, r) => l.query_vars().union(&r.query_vars()).copied().collect(),
            BasicOp::Minus(l, r) => l.query_vars().union(&r.query_vars()).copied().collect(),
            BasicOp::Multiply(l, r) => l.query_vars().union(&r.query_vars()).copied().collect(),
            BasicOp::Divide(l, r) => l.query_vars().union(&r.query_vars()).copied().collect(),
            BasicOp::Negate(l) => l.query_vars(),
        }
    }

    fn to_latex(&self, runtime: &dyn Runtime) -> Result<String, Error> {
        match self {
            BasicOp::Plus(l, r) => {
                let l = l.to_latex(runtime)?;
                let r = r.to_latex(runtime)?;
                Ok(format!("{{{}}}+{{{}}}", l, r))
            }
            BasicOp::Minus(l, r) => {
                let l = l.to_latex(runtime)?;
                let r = r.to_latex(runtime)?;
                Ok(format!("{{{}}}-{{{}}}", l, r))
            }
            BasicOp::Multiply(l, r) => {
                let l = l.to_latex(runtime)?;
                let r = r.to_latex(runtime)?;
                Ok(format!("{{{}}}\\cdot{{{}}}", l, r))
            }
            BasicOp::Divide(l, r) => {
                let l = l.to_latex(runtime)?;
                let r = r.to_latex(runtime)?;
                Ok(format!("{{{}}}\\over{{{}}}", l, r))
            }
            BasicOp::Negate(r) => {
                let r = r.to_latex(runtime)?;
                Ok(format!("-{{{}}}", r))
            }
        }
    }
}

#[derive(Debug)]
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

    fn query_vars(&self) -> HashSet<&str> {
        self.args
            .iter()
            .map(|a| a.query_vars())
            .fold(HashSet::new(), |acc, vars| {
                acc.union(&vars).copied().collect()
            })
    }

    fn to_latex(&self, runtime: &dyn Runtime) -> Result<String, Error> {
        let args = self
            .args
            .iter()
            .map(|a| a.to_latex(runtime))
            .collect::<Result<Vec<_>, _>>()?;
        runtime.to_latex(&self.name, &args)
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
        ["sin", "cos", "pow", "exp", "sqrt", "ln", "abs"]
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
                    Err(Error::Math("Sqrt of negative".to_owned()))
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
                    Err(Error::Math("Log of negative".to_owned()))
                } else {
                    Ok(args[0].ln())
                }
            }
            "abs" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "abs".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else {
                    Ok(args[0].abs())
                }
            }
            _ => Err(Error::UndefinedFunction(name.to_string())),
        }
    }

    fn to_latex(&self, name: &str, args: &[String]) -> Result<String, Error> {
        match name {
            "sin" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "sin".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else {
                    Ok(format!("sin({{{}}})", args[0]))
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
                    Ok(format!("cos({{{}}})", args[0]))
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
                    Ok(format!("({{{}}})^{{{}}}", args[0], args[1]))
                }
            }
            "sqrt" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "sqrt".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else {
                    Ok(format!("\\sqrt{{{}}}", args[0]))
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
                    Ok(format!("e^{{{}}}", args[0]))
                }
            }
            "ln" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "ln".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else {
                    Ok(format!("ln({{{}}})", args[0]))
                }
            }
            "abs" => {
                if args.len() != 1 {
                    Err(Error::InvalidArgCount {
                        op_name: "abs".to_string(),
                        got_args: args.len(),
                        expected_args: 1,
                    })
                } else {
                    Ok(format!("|{{{}}}|", args[0]))
                }
            }
            _ => Err(Error::UndefinedFunction(name.to_string())),
        }
    }
}
