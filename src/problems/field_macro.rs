macro_rules! mkstruct {
    (@TRANSLATE [INT $cur_name:ident] -> $struct_name:ident $($name:ident : $type:ty),*) => {
        struct $struct_name {
            pub cur_name: i32,
            $(pub $name : $type),*
        }
    };

    (@TRANSLATE [FLOAT $cur_name:ident] -> $struct_name:ident $($name:ident : $type:ty),*) => {
        #[derive(Debug,Default)]
        struct $struct_name {
            pub $cur_name: f32,
            $(pub $name : $type),*
        }
    };

    (@TRANSLATE [FLOAT $cur_name:ident], $($next:tt),* -> $struct_name:ident $($name:ident : $type:ty),*) => {
        mkstruct!(@TRANSLATE $($next),* -> $struct_name $cur_name : f32 $($name : $type),*);
    };
    (@TRANSLATE [INT $cur_name:ident], $($next:tt),* -> $struct_name:ident $($name:ident : $type:ty),*) => {
        mkstruct!(@TRANSLATE $($next),* -> $struct_name $cur_name : i32 $($name : $type),*);
    };

    ($struct_name:ident $($next:tt),*) => {
        mkstruct!(@TRANSLATE $($next),* -> $struct_name )
    };
}

macro_rules! make_problem {

    (@STRUCTDEF NUM=[NAME=$cur_name:ident, TYPE=$cur_type:ty] -> $problem_name:ident $($name:ident : $type:ty),*) => {
        struct $problem_name {
            fields: HashMap<String,String>,
            runtime: Box<dyn $crate::mathparse::Runtime>,

            pub $cur_name : Option<$cur_type>,
            $(pub $name: Option<$type>),*
        }
    };
    (@STRUCTDEF FUNC=[NAME=$cur_name:ident, ALLOWED_VARS=$vars:tt] -> $problem_name:ident $($name:ident : $type:ty),*) => {
        struct $problem_name {
            fields: std::collections::HashMap<String,String>,
            runtime: Box<dyn $crate::mathparse::Runtime>,

            pub $cur_name : Option<Box<dyn $crate::mathparse::Expression>>,
            $(pub $name: Option<$type>),*
        }
    };
    (@STRUCTDEF NUM=[NAME=$cur_name:ident, TYPE=$cur_type:ty], $($kind:ident=$desc:tt),* -> $problem_name:ident $($name:ident : $type:ty),*) => {
        make_problem!(@STRUCTDEF $($kind=$desc),* -> $problem_name $cur_name : $cur_type $($name : $type),* )
    };
    (@STRUCTDEF FUNC=[NAME=$cur_name:ident, ALLOWED_VARS=$vars:tt], $($kind:ident=$desc:tt),* -> $problem_name:ident $($name:ident : $type:ty),*) => {
        make_problem!(@STRUCTDEF $($kind=$desc),* -> $problem_name $cur_name : Box<dyn $crate::mathparse::Expression> $($name : $type),* )
    };

    (@IMPL_GET_NAMES NUM=[NAME=$cur_name:ident, TYPE=$cur_type:ty] -> $($names:ident),*) => {
        vec![stringify!($cur_name).to_string(), $($names.to_string()),*]
    };
    (@IMPL_GET_NAMES FUNC=[NAME=$cur_name:ident, ALLOWED_VARS=$vars:tt] -> $($names:ident),*) => {
        vec![stringify!($cur_name).to_string(), $(stringify!($names).to_string()),*]
    };
    (@IMPL_GET_NAMES NUM=[NAME=$cur_name:ident, TYPE=$cur_type:ty], $($kind:ident=$desc:tt),* -> $($names:ident),*) => {
        make_problem!(@IMPL_GET_NAMES $($kind=$desc),* -> $cur_name $($names),*)
    };
    (@IMPL_GET_NAMES FUNC=[NAME=$cur_name:ident, ALLOWED_VARS=$vars:tt], $($kind:ident=$desc:tt),* -> $($names:ident),*) => {
        make_problem!(@IMPL_GET_NAMES $($kind=$desc),* -> $cur_name $($names),*)
    };

    (@VALIDATE FUNC=[NAME=$cur_name:ident, ALLOWED_VARS=[$($var:literal),*]] $self:ident $errors:ident) => {
        $self.$cur_name = match $self.fields.get(stringify!($cur_name)) {
            None => {
                $errors.push($crate::view::ValidationError(format!("no such field {}",stringify!($cur_name))));
                None
            },
            Some(val) => {
                match parse(val, $self.runtime.as_ref()) {
                    Some(expr) => {
                        let vars = expr.query_vars();
                        let allowed = std::collections::HashSet::from(&[$($var),*]);
                        if !vars.iter().all(|v| allowed.contains(v)) {
                            $errors.push($crate::view::ValidationError(format!("{}: got variables {:?}, allowed: {}",stringify!($cur_name), vars, stringify!([$($var),*]))));
                            None
                        } else {
                            Some(expr)
                        }
                    },
                    None => {
                        $errors.push($crate::view::ValidationError(format!("{}: could not parse",stringify!($cur_name))));
                        None
                    }
                }
            }
        };
    };
    (@VALIDATE NUM=[NAME=$cur_name:ident, TYPE=$cur_type:ty] $self:ident $errors:ident) => {
        $self.$cur_name = match $self.fields.get(stringify!($cur_name)) {
            None => {
                $errors.push($crate::views::ValidationError(format!("no such field {}",stringify!($cur_name))));
                None
            },
            Some(val) => match val.parse::<$cur_type>() {
                Ok(num) => Some(num),
                Err(e) => {
                    $errors.push($crate::views::ValidationError(format!("{}: could not parse - {:?}",stringify!($cur_name), e)));
                    None
                }
            }
        };
    };

    (@VALIDATE NUM=[NAME=$cur_name:ident, TYPE=$cur_type:ty] -> $self:ident $($names:ident),*) => {
        match name {
            stringify!($cur_name) => {
                make_problem!(@VALIDATE NUM=[NAME=$cur_name, TYPE=$cur_type] $self)
            },
        }
    };
    (@VALIDATE FUNC=[NAME=$cur_name:ident, ALLOWED_VARS=$vars:tt] -> $self:ident $($names:ident),*) => {
    };
    (@VALIDATE NUM=[NAME=$cur_name:ident, TYPE=$cur_type:ty], $($kind:ident=$desc:tt),* -> $self:ident $($names:ident),*) => {
    };
    (@VALIDATE FUNC=[NAME=$cur_name:ident, ALLOWED_VARS=$vars:tt], $($kind:ident=$desc:tt),* -> $self:ident $($names:ident),*) => {
    };

    (STRUCT $problem_name: ident = [$($kind:ident = $desc:tt),*]) => {
        make_problem!(@STRUCTDEF $($kind = $desc),* -> $problem_name);

        impl $crate::views::Problem for $problem_name {
            fn get_about(&self) -> String {
                todo!()
            }
            fn get_field_names(&self) -> Vec<String>{
                make_problem!(@IMPL_GET_NAMES $($kind = $desc),* -> )
            }
            fn get_field_val(&self, name: &str) -> Option<String>{
                self.fields.get(name).cloned()
            }
            fn set_field(&mut self, name: &str, new_val:String) -> Option<$crate::views::ValidationError>{
                todo!()
            }

            fn solve(&self) -> Vec<$crate::views::SolutionParagraph>{
                todo!()
            }
        }
    };
}

#[test]
fn hi() {
    make_problem!(STRUCT MyProblem = [
        NUM = [NAME = myfloat, TYPE = f64],
        FUNC = [NAME = myfunc, ALLOWED_VARS=["x","y"]]
    ]);
}
