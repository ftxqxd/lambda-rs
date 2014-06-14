#![feature(phase)]
#![allow(experimental)]

extern crate lambda;
#[phase(plugin, link)]
extern crate log;

use std::io::stdin;
use lambda::LambdaExpr;

fn main() {
    let expr: LambdaExpr = from_str(stdin().read_line().unwrap().as_slice()).unwrap();
    debug!("Input: {}", expr);
    debug!("Repr:  {}", expr.repr());
    // debug!("Alpha: {}", expr.clone().alpha_rename());
    // debug!("Beta:  {}", expr.clone().alpha_rename().beta_reduce());
    // debug!("Eta:   {}", expr.clone().alpha_rename().eta_convert());
    // debug!("All:   {}", expr.clone().reduce());
    println!("{}", expr.reduce());
}
