#[crate_id = "lambda#0.1"];
#[crate_type = "lib"];
#[feature(macro_rules)];

// TODO: fix (func \l.l 0 0)

mod parse;

macro_rules! str {
    ($a:expr) => {
        format!("{}", $a)
    }
}

#[deriving(Eq, Clone, Hash)]
pub enum LambdaExpr {
    Nothing,
    Variable(~str, i16),
    Call(~LambdaExpr, ~LambdaExpr),
    Lambda(~str, i16, ~LambdaExpr),
}

impl LambdaExpr {
    pub fn traverse(&self, f: |&LambdaExpr|) {
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
    pub fn traverse_mut(&mut self, f: |&mut LambdaExpr|) {
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
    pub fn beta_reduce(&self) -> LambdaExpr {
        let result = self.clone();
        match result {
            Call(~Lambda(name, id, ~e), ~f) => {
                let mut new_e = e;
                new_e.traverse_mut(|ex| {
                    if *ex == Variable(name.to_owned(), id) {
                        *ex = f.clone(); // Is this the problem?
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
    fn from_str(s: &str) -> Option<LambdaExpr> {
        parse::parse(&mut parse::tokenise(s).move_iter()).ok()
    }
}

impl IntoStr for LambdaExpr {
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
