use super::{LambdaExpr, Variable, Lambda, Call};

#[test]
fn test_parse() {
    let e: Option<LambdaExpr> = from_str("\\x.(\\y z.(x y z))");
    assert_eq!(e, Some(Lambda("x".to_string(), 0, box Lambda("y".to_string(), 0, box Lambda("z".to_string(), 0,
        box Call(box Call(box Variable("x".to_string(), 0), box Variable("y".to_string(), 0)),
            box Variable("z".to_string(), 0)))))));
}

#[test]
fn test_alpha_rename() {
    assert_eq!(Call(box Lambda("x".to_string(), 0i16, box Call(box Lambda("x".to_string(), 0i16,
                box Variable("x".to_string(), 0i16)), box Variable("x".to_string(), 0i16))),
                box Variable("x".to_string(), 0i16)).alpha_rename(),
               Call(box Lambda("x".to_string(), 1i16, box Call(box Lambda("x".to_string(), 2i16,
                box Variable("x".to_string(), 2i16)), box Variable("x".to_string(), 1i16))),
                box Variable("x".to_string(), 0i16)));
}

#[test]
fn test_beta_reduce() {
    assert_eq!(Call(box Lambda("x".to_string(), 0i16, box Call(box Variable("f".to_string(), 0i16),
                box Variable("x".to_string(), 0i16))),
                box Variable("a".to_string(), 0i16)).beta_reduce(),
               Call(box Variable("f".to_string(), 0i16), box Variable("a".to_string(), 0i16)));
}

// #[test]
// fn test_eta_convert() {
//     assert_eq!(Lambda("x".to_string(), 0i16, box Call(box Variable("f".to_string(), 0i16),
//                 box Variable("x".to_string(), 0i16))).eta_convert(),
//                Variable("f".to_string(), 0i16));
// }

#[test]
fn test_reduce() {
    assert_eq!(Call(box Lambda("f".to_string(), 0i16, box Call(box Lambda("f".to_string(), 0i16,
                box Lambda("x".to_string(), 0i16, box Call(box Variable("f".to_string(), 0i16),
                box Variable("x".to_string(), 0i16)))), box Variable("f".to_string(), 0i16))),
                box Variable("a".to_string(), 0i16)).reduce(),
               Variable("a".to_string(), 0i16));
}
