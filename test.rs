extern crate lambda;

use lambda::{LambdaExpr, Variable, Lambda, Call};

#[test]
fn test_parse() {
    let e: Option<LambdaExpr> = from_str("\\x.(\\y z.(x y z))");
    assert_eq!(e, Some(Lambda(~"x", 0, ~Lambda(~"y", 0, ~Lambda(~"z", 0,
        ~Call(~Call(~Variable(~"x", 0), ~Variable(~"y", 0)),
            ~Variable(~"z", 0)))))));
}
