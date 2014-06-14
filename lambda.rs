#![crate_id = "lambda#0.2"]
#![crate_type = "lib"]
#![feature(macro_rules, phase)]

//! A basic parser/reducer for the λ-calculus.
//!
//! To parse a λ-calculus expression, use
//! [`LambdaExpr::`](enum.LambdaExpr.html)[`from_str`](enum.LambdaExpr.html#method.from_str).
//! Reducing a λ-calculus expression is done by using
//! [`LambdaExpr::`](enum.LambdaExpr.html)[`reduce`](enum.LambdaExpr.html#method.reduce).
//! To convert a [`LambdaExpr`](enum.LambdaExpr.html) to a `String`, use
//! [`LambdaExpr::`](enum.LambdaExpr.html)[`into_str`](enum.LambdaExpr.html#method.into_str).
//!
//! Example usage:
//!
//! ~~~
//! extern crate lambda;
//! 
//! use std::io::stdin;
//! use lambda::LambdaExpr;
//! 
//! fn main() {
//!     let expr: Option<LambdaExpr> = from_str(stdin().read_line().unwrap());
//!     println!("{}", expr.unwrap().reduce().into_str());
//! }
//! ~~~

#[phase(plugin, link)]
extern crate log;

use std::from_str::FromStr;
use std::fmt;
use std::hash::hash;

// TODO: fix `func \l.l 0 0`

mod parse;
#[cfg(test)]
mod test;

/// A λ-calculus expression.
#[deriving(PartialEq, Eq, Clone, Hash)]
pub enum LambdaExpr {
    /// Nothing represents an empty λ-calculus expression.
    ///
    /// This can only be found as an empty program or inside an empty
    /// lambda body.
    Nothing,
    /// A variable (bound or unbound).
    ///
    /// The integer field represents the internal identifier for the
    /// variable (generated during α-reduction). This is 0 for unbound
    /// variables.
    Variable(String, i16),
    /// An expression called with another expression.
    Call(Box<LambdaExpr>, Box<LambdaExpr>),
    /// An anonymous function (lambda).
    ///
    /// The integer field represents the internal identifier for the
    /// parameter (generated during α-reduction).
    Lambda(String, i16, Box<LambdaExpr>),
}

impl LambdaExpr {
    fn traverse_mut(&mut self, f: |&mut LambdaExpr|) {
        match *self {
            Nothing => (),
            Variable(_, _) => (),
            Call(ref mut a, ref mut b) => {
                a.traverse_mut(|x| f(x));
                b.traverse_mut(|x| f(x));
            },
            Lambda(_, _, ref mut e) => {
                e.traverse_mut(|x| f(x));
            },
        };
        if *self != Nothing { f(self) }
    }

    /// α-rename a λ-expression, consuming the original.
    ///
    /// α-renaming is the process of renaming variables to prevent name
    /// conflicts.
    ///
    /// This will not actually rename any variables; it will only change
    /// their internal identifier (the integer field in `Lambda`s and
    /// `Variable`s).
    pub fn alpha_rename(self) -> LambdaExpr {
        self.alpha_rename_n(1).val0()
    }

    fn alpha_rename_n(self, mut n: i16) -> (LambdaExpr, i16) {
        (match self {
            Lambda(name, _, e) => {
                let mut new_e = e;
                new_e.traverse_mut(|ex| {
                    let var = match *ex {
                        Variable(_, _) => true,
                        _ => false,
                    };
                    if var {
                        let mut new = None;
                        match *ex {
                            Variable(ref name2, _) =>
                                if name2.as_slice() == name.as_slice() {
                                    new = Some(Variable(name.clone(), n));
                                },
                            _ => (),
                        }
                        match new {
                            Some(val) => *ex = val,
                            None => {},
                        }
                    }
                });
                let oldn = n;
                let (ee, en) = new_e.alpha_rename_n(n + 1);
                n = en;
                Lambda(name, oldn, box ee)
            },
            Call(a, b) => {
                let (a_e, a_n) = a.alpha_rename_n(n);
                let (b_e, b_n) = b.alpha_rename_n(a_n);
                n = b_n;
                Call(box a_e, box b_e)
            },
            _ => self,
        }, n)
    }

    /// β-reduce a λ-expression, consuming the original.
    ///
    /// β-reduction is the process of converting `(λx.e) a` to `e[x := a]`
    /// (i.e. replacing all occurences of the variable `x` in `e` with `a`).
    pub fn beta_reduce(self) -> LambdaExpr {
        match self {
            Call(box Lambda(name, id, box e), box f) => {
                let mut e = e.beta_reduce();
                e.traverse_mut(|ex| {
                    if *ex == Variable(name.to_string(), id) {
                        *ex = f.clone();
                    }
                });
                e
            },
            Call(a, b) => {
                Call(box a.beta_reduce(), box b.beta_reduce())
            },
            Lambda(name, id, e) => {
                Lambda(name, id, box e.beta_reduce())
            },
            _ => self,
        }
    }

    /// η-convert a λ-expression, consuming the original.
    ///
    /// η-conversion is the process of converting `(λx.f x)` to `f`.

    // TODO: fix bad behaviour
    #[experimental = "buggy (`(\\x.x x)` converts to `x`)"]
    pub fn eta_convert(self) -> LambdaExpr {
        #![allow(experimental)]
        // return self;
        match self {
            Lambda(name, id, box Call(e, box Variable(name2, id2))) =>
                if name == name2 && id == id2 {
                    *e
                } else {
                    Lambda(name, id, box Call(e, box Variable(name2, id2)))
                },
            Call(a, b) =>
                Call(box a.eta_convert(), box b.eta_convert()),
            Lambda(name, id, e) =>
                Lambda(name, id, box e.eta_convert()),
            _ => self,
        }
    }
    

    /// Reduce a λ-expression as much as possible, consuming the original.
    ///
    /// This is done by repeatedly calling `beta_reduce` until it makes no difference.
    pub fn reduce(self) -> LambdaExpr {
        let mut oldhash;
        let mut curr = self.alpha_rename();
        loop {
            debug!("Reduce: {}", curr);
            oldhash = hash(&curr);
            curr = curr.beta_reduce();
            if hash(&curr) == oldhash {
                // curr = curr.eta_convert();
                // if hash(&curr) == oldhash {
                break;
                // }
            }
        }
        curr
    }
}

impl FromStr for LambdaExpr {
    /// Create a new `LambdaExpr` by parsing a string.
    ///
    /// The grammar for such expressions is roughly as follows:
    ///
    /// box box box .notrust
    /// expr ::= <lambda> | <IDENT> | <call> | '(' <expr> ')'
    /// lambda ::= ('\\' | 'λ') <IDENT>+ '.' <expr>
    /// call ::= <expr> <expr>
    /// box box box 
    /// 
    /// Some constants are also provided:
    ///
    /// * Numbers (`1`, `2`, `3`, ...)
    /// * Booleans (`T`, `F`)
    /// * Successor function (`S`)
    /// * Zero conditional (`Z`)
    /// * Multiplication (`*`)
    /// * Logical operators (`&`, `|`, `!`)
    /// * [Fixed-point combinator](https://en.wikipedia.org/wiki/Fixed-point_combinator) (`Y`)
    fn from_str(s: &str) -> Option<LambdaExpr> {
        parse::parse(&mut parse::tokenise(s).iter().map(|x| x.as_slice())).ok()
    }
}

impl fmt::Show for LambdaExpr {
    /// Format a `LambdaExpr`.
    ///
    /// The syntax for such a string is the same as for `from_str`, but
    /// prefers to use `λ` in lambda declarations (rather than `\`);
    /// does not use special names for constants, using their fully expanded
    /// form instead; and fully parenthesises all expressions.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Nothing => write!(f, ""),
            Variable(ref a, _) => write!(f, "{}", a),
            Call(ref a, ref b) => write!(f, "({} {})", a, b),
            Lambda(ref name, _, ref expr) =>
                write!(f, "(λ{}.{})", name, expr),
        }
    }
}

impl LambdaExpr {
    pub fn repr(&self) -> String {
        match *self {
            Nothing => format!("Nothing"),
            Variable(ref a, id) => format!("Variable(\"{}\".to_string(), {}i16)", a, id),
            Call(ref a, ref b) => format!("Call(box {}, box {})", a.repr(), b.repr()),
            Lambda(ref name, id, ref expr) =>
                format!("Lambda(\"{}\".to_string(), {}i16, box {})", name, id, expr.repr()),
        }
    }
}