extern crate lambda;

use std::io::stdin;
use lambda::LambdaExpr;

fn main() {
    let expr: Option<LambdaExpr> = from_str(stdin().read_line().unwrap());
    // println!("{:?}", expr.clone().unwrap());
    println!("{}", expr.unwrap().reduce().into_str());
}
