use super::{LambdaExpr, Nothing, Variable, Call, Lambda};

macro_rules! str {
    ($a:expr) => {
        format!("{}", $a)
    }
}

pub fn tokenise(s: &str) -> ~[~str] { 
    let mut tokens: ~[~str] = ~[]; 
    let mut buf = ~""; 
    for c in s.chars() { 
        match c { 
            '\\' | 'λ' | '.' | '(' | ')' => { 
                if buf != ~"" { tokens.push(buf); } 
                buf = ~""; 
                tokens.push(str!(c)); 
            }, 
            ' ' | '\t' | '\n' | '\r' => { 
                if buf != ~"" { tokens.push(buf); } 
                buf = ~""; 
                continue; 
            }, 
            _ => { 
                buf = buf + str!(c); 
            }, 
        }; 
    } 
    if buf != ~"" { tokens.push(buf); } 
    tokens 
}

pub fn parse<T: Iterator<~str>>(toki: &mut T)
                            -> Result<LambdaExpr, (~str, LambdaExpr)> {
    let mut tok = toki.next();
    let mut res = Nothing;
    while tok.is_some() {
        let next = match tok {
            Some(~"\\") | Some(~"λ") => {
                let mut vars = ~[];
                let mut varname = toki.next();
                while varname.is_some() && varname != Some(~".") {
                    vars.push(varname.clone());
                    varname = toki.next();
                }
                let dot = varname.clone();
                if dot != Some(~".") {
                    return Err((format!("expected `.`, found `{}`",
                                        dot.unwrap_or(~"end of file")),
                               res));
                }
                let subexpr = parse(toki);
                let mut currexpr = match subexpr {
                    Ok(se) => se,
                    Err((msg, r)) => {
                        let mut rm = r;
                        for v in vars.iter().rev() {
                            match *v {
                                Some(_) => 
                                    rm = Lambda(v.clone().unwrap(), 0, ~rm),
                                None => return Err(
                                            (~"unexpected end of file", res)),
                            }
                        }
                        return Err((msg, rm));
                    },
                };
                for v in vars.iter().rev() {
                    match *v {
                        Some(_) => 
                            currexpr = Lambda(v.clone().unwrap(), 0, ~currexpr),
                        None => return Err(
                                    (~"unexpected end of file", res)),
                    }
                }
                currexpr
            },
            Some(~".") => return Err((~"unexpected `.`", res)),
            Some(~"(") => {
                let thing = parse(toki);
                let res2 = match thing {
                    Ok(x) => x,
                    Err((~"unexpected `)`", r)) => r,
                    Err(_) => return thing,
                };
                res2
            },
            Some(~")") => return Err((~"unexpected `)`", res)),
            Some(~"T") => Lambda(~"x", 0, ~Lambda(~"y", 0, ~Variable(~"x", 0))),
            Some(~"F") => Lambda(~"x", 0, ~Lambda(~"y", 0, ~Variable(~"y", 0))),
            Some(~"S") => Lambda(~"w", 0, ~Lambda(~"y", 0, ~Lambda(~"x", 0, ~Call(~Variable(~"y", 0), ~Call(~Call(~Variable(~"w", 0), ~Variable(~"y", 0)), ~Variable(~"x", 0)))))),
            Some(~"Z") => Lambda(~"x", 0i16, ~Call(~Call(~Call(~Variable(~"x", 0i16), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"y", 0i16)))), ~Lambda(~"x", 0i16, ~Call(~Call(~Variable(~"x", 0i16), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"y", 0i16)))), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"x", 0i16)))))), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"y", 0i16))))),
            Some(~"*") => Lambda(~"x", 0, ~Lambda(~"y", 0, ~Lambda(~"z", 0, ~Call(~Variable(~"x", 0), ~Call(~Variable(~"y", 0), ~Variable(~"z", 0)))))),
            Some(~"&") => Lambda(~"x", 0, ~Lambda(~"y", 0, ~Call(~Call(~Variable(~"x", 0), ~Variable(~"y", 0)), ~Lambda(~"x", 0, ~Lambda(~"y", 0, ~Variable(~"y", 0)))))),
            Some(~"|") => Lambda(~"x", 0, ~Lambda(~"y", 0, ~Call(~Call(~Variable(~"x", 0), ~Lambda(~"x", 0, ~Lambda(~"y", 0, ~Variable(~"x", 0)))), ~Variable(~"y", 0)))),
            Some(~"!") => Lambda(~"x", 0i16, ~Call(~Call(~Variable(~"x", 0i16), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"y", 0i16)))), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"x", 0i16))))),
            Some(~"Y") => Lambda(~"y", 0i16, ~Call(~Lambda(~"x", 0i16, ~Call(~Variable(~"y", 0i16), ~Call(~Variable(~"x", 0i16), ~Variable(~"x", 0i16)))), ~Lambda(~"x", 0i16, ~Call(~Variable(~"y", 0i16), ~Call(~Variable(~"x", 0i16), ~Variable(~"x", 0i16)))))),
            // XXX: remove when `P` works
            // Some(~"func") => Lambda(~"p", 0i16, ~Lambda(~"z", 0i16, ~Call(~Call(~Variable(~"z", 0i16), ~Lambda(~"y", 0i16, ~Lambda(~"x", 0i16, ~Call(~Variable(~"y", 0i16), ~Call(~Call(~Call(~Variable(~"p", 0i16), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"x", 0i16)))), ~Variable(~"y", 0i16)), ~Variable(~"x", 0i16)))))), ~Call(~Variable(~"p", 0i16), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"x", 0i16))))))),
            // VERY STRANGE, does not work
            // Some(~"P") => Lambda(~"n", 0i16, ~Call(~Call(~Call(~Variable(~"n", 0i16), ~Lambda(~"p", 0i16, ~Lambda(~"l", 0i16, ~Call(~Call(~Variable(~"l", 0i16), ~Call(~Variable(~"p", 0i16), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"y", 0i16))))), ~Call(~Lambda(~"w", 0i16, ~Lambda(~"y", 0i16, ~Lambda(~"x", 0i16, ~Call(~Variable(~"y", 0i16), ~Call(~Call(~Variable(~"w", 0i16), ~Variable(~"y", 0i16)), ~Variable(~"x", 0i16)))))), ~Call(~Variable(~"p", 0i16), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"y", 0i16))))))))), ~Lambda(~"z", 0i16, ~Call(~Call(~Variable(~"z", 0i16), ~Lambda(~"s", 0i16, ~Lambda(~"z", 0i16, ~Variable(~"z", 0i16)))), ~Lambda(~"s", 0i16, ~Lambda(~"z", 0i16, ~Variable(~"z", 0i16)))))), ~Lambda(~"x", 0i16, ~Lambda(~"y", 0i16, ~Variable(~"y", 0i16))))),
            Some(ref num) if from_str::<uint>(num.clone()).is_some() => {
                let nm = from_str::<int>(num.clone()).unwrap();
                let mut numb = Variable(~"z", 0);
                for _ in range(0, nm) {
                    numb = Call(~Variable(~"s", 0), ~numb);
                }
                Lambda(~"s", 0, ~Lambda(~"z", 0, ~numb))
            },
            Some(name) => Variable(name, 0),
            None => return Err((~"", res)),
        };
        res = if res == Nothing {
                  next
              } else {
                  Call(~res, ~next)
              };
        tok = toki.next();
    }
    Ok(res)
}
