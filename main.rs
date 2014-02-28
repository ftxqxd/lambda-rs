extern crate lambda;

use std::io::stdin;
use lambda::LambdaExpr;

fn main() {
    let code = stdin().read_line().unwrap();
    let parsed: LambdaExpr = from_str(code).unwrap();
    //println!("{:?}", parsed.clone());
    let converted = parsed.clone().reduce();
    println!("{}", converted.clone().into_str());
}
