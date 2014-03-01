extern crate lambda;

use std::io::stdin;
use lambda::LambdaExpr;

fn main() {
    let expr: LambdaExpr = from_str(stdin().read_line().unwrap()).unwrap();
    debug!("{:?}", expr);
    debug!("Alpha: {:?}", expr.alpha_rename());
    debug!("Beta:  {:?}", expr.alpha_rename().beta_reduce());
    debug!("Eta:   {:?}", expr.alpha_rename().eta_convert());
    debug!("All:   {:?}", expr.reduce());
    println!("{}", expr.reduce().into_str());
}
