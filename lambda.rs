#[crate_id = "lambda#0.1"];
#[crate_type = "lib"];
#[feature(macro_rules)];

//! A basic parser/reducer for the λ-calculus.
//!
//! To parse a λ-calculus expression, use `from_str` (re-exported in
//! `std::prelude`). Reducing a λ-calculus expression is done using
//! `LambdaExpr::reduce`. To convert a `LambdaExpr` to a `~str`, use
//! `LambdaExpr::into_str`.
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

// TODO: fix (func \l.l 0 0)

mod parse;

macro_rules! str {
    ($a:expr) => {
        format!("{}", $a)
    }
}

/// A λ-calculus expression.
#[deriving(Eq, Clone)]
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
    Variable(~str, i16),
    /// An expression called with another expression.
    Call(~LambdaExpr, ~LambdaExpr),
    /// An anonymous function (lambda).
    ///
    /// The integer field represents the internal identifier for the
    /// parameter (generated during α-reduction).
    Lambda(~str, i16, ~LambdaExpr),
}

impl LambdaExpr {
    fn traverse(&self, f: |&LambdaExpr|) {
        match *self {
            Nothing => (),
            Variable(_, _) => (),
            Call(ref a, ref b) => {
                a.traverse(|x| f(x));
                b.traverse(|x| f(x));
            },
            Lambda(_, _, ref e) => e.traverse(|x| f(x)),
        };
        if *self != Nothing { f(self) }
    }
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
    /// Create a new, α-renamed version of a λ-expression.
    ///
    /// Α-renaming is the process of renaming variables to prevent name
    /// conflicts.
    ///
    /// This will not actually rename any variables; it will only change
    /// their internal identifier (the integer field in `Lambda`s and
    /// `Variable`s).
    pub fn alpha_rename(&self) -> LambdaExpr {
        self.alpha_rename_n(1).val0()
    }
    fn alpha_rename_n(&self, mut n: i16) -> (LambdaExpr, i16) {
        let result = self.clone();
        (match result {
            Lambda(name, _, e) => {
                let mut new_e = e;
                new_e.traverse_mut(|ex| {
                    let var = match *ex {
                        Variable(_, _) => true,
                        _ => false,
                    };
                    if var {
                        match ex.clone() {
                            Variable(name2, _) =>
                                if name2 == name.to_owned() {
                                    *ex = Variable(name.to_owned(), n);
                                },
                            _ => (),
                        }
                    }
                });
                let oldn = n;
                let (ee, en) = new_e.alpha_rename_n(n + 1);
                n = en;
                Lambda(name, oldn, ~ee)
            },
            Call(a, b) => {
                let (a_e, a_n) = a.alpha_rename_n(n);
                let (b_e, b_n) = b.alpha_rename_n(a_n);
                n = b_n;
                Call(~a_e, ~b_e)
            },
            _ => result,
        }, n)
    }
    /// Create a new, β-reduced version of a λ-expression.
    ///
    /// Β-reduction is the process of converting `(λx.e) a` to `e[x := a]`
    /// (i.e. replacing all occurences of the variable `x` in `e` with `a`).
    pub fn beta_reduce(&self) -> LambdaExpr {
        let result = self.clone();
        match result {
            Call(~Lambda(name, id, ~e), ~f) => {
                let mut new_e = e;
                new_e.traverse_mut(|ex| {
                    if *ex == Variable(name.to_owned(), id) {
                        *ex = f.clone();
                    }
                });
                new_e
            },
            Call(a, b) => {
                Call(~a.beta_reduce(), ~b.beta_reduce())
            },
            Lambda(name, id, e) => {
                Lambda(name, id, ~e.beta_reduce())
            },
            _ => result,
        }
    }
    /// Create a new, η-converted version of a λ-expression.
    ///
    /// Η-conversion is the process of converting `(λx.f x)` to `f`.
    pub fn eta_convert(&self) -> LambdaExpr {
        let result = self.clone();
        match result {
            Lambda(name, id, ~Call(e, ~Variable(name2, id2))) => {
                if name == name2 && id == id2 { return *e; }
            },
            Call(a, b) => {
                return Call(~a.eta_convert(), ~b.eta_convert());
            },
            Lambda(name, id, e) => {
                return Lambda(name, id, ~e.eta_convert());
            },
            _ => return result,
        };
        self.clone()
    }
    /// Reduce a λ-expression as much as possible.
    ///
    /// This is done by repeatedly calling `beta_reduce` unless it makes no
    /// difference, in which case it calls `eta_reduce`. If that makes no
    /// difference, return; otherwise, repeat.
    pub fn reduce(&self) -> LambdaExpr {
        // TODO: make less terrible (too much `.clone()`!)
        let mut oldcurr;
        let mut curr = self.alpha_rename();
        // Is this really the best way to do it?
        loop {
            oldcurr = curr.clone();
            curr = curr.beta_reduce();
            if curr == oldcurr {
                curr = curr.eta_convert();
                if curr == oldcurr {
                    break;
                }
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
    /// ~~~.notrust
    /// expr ::= <lambda> | <IDENT> | <call> | '(' <expr> ')'
    /// lambda ::= ('\\' | 'λ') <IDENT>+ '.' <expr>
    /// call ::= <expr> <expr>
    /// ~~~
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
        parse::parse(&mut parse::tokenise(s).move_iter()).ok()
    }
}

impl IntoStr for LambdaExpr {
    /// Convert a `LambdaExpr` into a string, consuming it in the process.
    ///
    /// The syntax for such a string is the same as for `from_str`, but
    /// prefers to use `λ` in lambda declarations (rather than `\`) and
    /// does not use special names for constants, using their fully expanded
    /// form instead.
    fn into_str(self) -> ~str {
        match self {
            Nothing => ~"",
            Variable(a, _) => a,
            Call(a, b) => format!("({} {})", a.into_str(), b.into_str()),
            Lambda(name, _, expr) =>
                format!("(λ{}.{})", name, expr.into_str()),
        }
    }
}
