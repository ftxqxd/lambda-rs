extern crate lambda;

#[cfg(test)]
mod test {
    use lambda::{LambdaExpr, Variable, Lambda, Call};

    #[test]
    fn test_parse() {
        let e: Option<LambdaExpr> = from_str("\\x.(\\y z.(x y z))");
        assert_eq!(e, Some(Lambda(~"x", 0, ~Lambda(~"y", 0, ~Lambda(~"z", 0,
            ~Call(~Call(~Variable(~"x", 0), ~Variable(~"y", 0)),
                ~Variable(~"z", 0)))))));
    }

    #[test]
    fn test_alpha_rename() {
        assert_eq!(Call(~Lambda(~"x", 0i16, ~Call(~Lambda(~"x", 0i16,
                    ~Variable(~"x", 0i16)), ~Variable(~"x", 0i16))),
                    ~Variable(~"x", 0i16)).alpha_rename(),
                   Call(~Lambda(~"x", 1i16, ~Call(~Lambda(~"x", 2i16,
                    ~Variable(~"x", 2i16)), ~Variable(~"x", 1i16))),
                    ~Variable(~"x", 0i16)));
    }

    #[test]
    fn test_beta_reduce() {
        assert_eq!(Call(~Lambda(~"x", 0i16, ~Call(~Variable(~"f", 0i16),
                    ~Variable(~"x", 0i16))),
                    ~Variable(~"a", 0i16)).beta_reduce(),
                   Call(~Variable(~"f", 0i16), ~Variable(~"a", 0i16)));
    }
    
    #[test]
    fn test_eta_convert() {
        assert_eq!(Lambda(~"x", 0i16, ~Call(~Variable(~"f", 0i16),
                    ~Variable(~"x", 0i16))).eta_convert(),
                   Variable(~"f", 0i16));
    }

    #[test]
    fn test_reduce() {
        assert_eq!(Call(~Lambda(~"f", 0i16, ~Call(~Lambda(~"f", 0i16,
                    ~Lambda(~"x", 0i16, ~Call(~Variable(~"f", 0i16),
                    ~Variable(~"x", 0i16)))), ~Variable(~"f", 0i16))),
                    ~Variable(~"a", 0i16)).reduce(),
                   Variable(~"a", 0i16));
    }
}
