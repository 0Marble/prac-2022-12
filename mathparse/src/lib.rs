mod expr;
mod parse;

pub use expr::*;
use parse::*;

pub fn parse(expr: &str, language: &dyn Runtime) -> Option<Box<dyn Expression>> {
    tokenize(expr).and_then(|tokens| parse_expr(&tokens, language))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_func() {
        let expr = "1+2";
        let lang = DefaultRuntime::default();

        assert_eq!(parse(expr, &lang).map(|e| e.eval(&lang)), Some(Ok(3.0)));

        let expr = "x-10";
        assert_eq!(
            parse(expr, &lang).map(|e| e.eval(&DefaultRuntime::new(&[("x", 10.0)]))),
            Some(Ok(0.0))
        );

        let expr = "122+904-23.1*(72-x/4)";
        assert_eq!(
            parse(expr, &lang).map(|e| e.eval(&DefaultRuntime::new(&[("x", 8.0)]))),
            Some(Ok(122.0 + 904.0 - 23.1 * (72.0 - 8.0 / 4.0)))
        );
    }

    #[test]
    fn order_of_ops() {
        assert_eq!(
            parse("1/2/3", &DefaultRuntime::default()).map(|e| e.eval(&DefaultRuntime::default())),
            Some(Ok(1.0 / 2.0 / 3.0))
        );

        assert_eq!(
            parse("1-2-3", &DefaultRuntime::default()).map(|e| e.eval(&DefaultRuntime::default())),
            Some(Ok(1.0 - 2.0 - 3.0))
        );
    }

    #[test]
    fn implicit_multiplication() {
        let x = 2.0;
        let y = -1.2;
        let lang = DefaultRuntime::default();
        assert_eq!(
            parse("2x", &lang).map(|e| e.eval(&DefaultRuntime::new(&[("x", x)]))),
            Some(Ok(4.0))
        );

        assert_eq!(
            parse("2sin(x)-3cos(4x)", &lang).map(|e| e.eval(&DefaultRuntime::new(&[("x", x)]))),
            Some(Ok(2.0 * f64::sin(2.0) - 3.0 * f64::cos(4.0 * 2.0)))
        );

        assert_eq!(
            parse(
                "-sin((5-3)cos(2.1x-sqrt(3+2-0.2x))+3pow(6,2y))-pow(1.1,-(10-y)x+y)",
                &lang
            )
            .map(|e| e.eval(&DefaultRuntime::new(&[("x", x), ("y", y)]))),
            Some(Ok(-f64::sin(
                (5.0 - 3.0) * f64::cos(2.1 * x - f64::sqrt(3.0 + 2.0 - 0.2 * x))
                    + 3.0 * f64::powf(6.0, 2.0 * y)
            ) - f64::powf(1.1, -(10.0 - y) * x + y)))
        );
    }

    #[test]
    fn vars() {
        let expr = "x+4(x-2y)sin(z*x)";
        let lang = DefaultRuntime::default();
        let expr = parse(expr, &lang).unwrap();
        let vars = expr.query_vars();
        dbg!(&vars);
        assert!(vars.len() == 3 && vars.contains("x") && vars.contains("y") && vars.contains("z"));
    }
}
