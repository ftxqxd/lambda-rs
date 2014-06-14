use super::{LambdaExpr, Nothing, Variable, Call, Lambda};

macro_rules! str {
    ($a:expr) => {
        format!("{}", $a)
    }
}

pub fn tokenise(s: &str) -> Vec<String> {
    let mut tokens = vec![];
    let mut buf = "".to_string();
    for c in s.chars() {
        match c {
            '\\' | 'λ' | '.' | '(' | ')' => {
                if buf != String::new() { tokens.push(buf); }
                buf = String::new();
                tokens.push(format!("{}", c));
            },
            ' ' | '\t' | '\n' | '\r' => {
                if buf != String::new() { tokens.push(buf); }
                buf = String::new();
                continue;
            },
            _ => {
                buf = buf.append(str!(c).as_slice());
            },
        };
    }
    if buf != String::new() { tokens.push(buf); }
    tokens
}

pub fn parse<'a, T: Iterator<&'a str>>(toki: &mut T)
                                  -> Result<LambdaExpr, (String, LambdaExpr)> {
    let mut tok = toki.next();
    let mut res = Nothing;
    while tok.is_some() {
        let next = match tok {
            Some("\\") | Some("λ") => {
                let mut vars = vec![];
                let mut varname = toki.next();
                while varname.is_some() && varname != Some(".") {
                    vars.push(varname.clone());
                    varname = toki.next();
                }
                let dot = varname.clone();
                if dot != Some(".") {
                    return Err((format!("expected `.`, found `{}`",
                                        dot.unwrap_or("end of file")),
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
                                    rm = Lambda(v.clone().unwrap().to_string(), 0, box rm),
                                None => return Err(
                                            ("unexpected end of file".to_string(), res)),
                            }
                        }
                        return Err((msg, rm));
                    },
                };
                for v in vars.iter().rev() {
                    match *v {
                        Some(_) =>
                            currexpr = Lambda(v.clone().unwrap().to_string(), 0, box currexpr),
                        None => return Err(
                                    ("unexpected end of file".to_string(), res)),
                    }
                }
                currexpr
            },
            Some(".") => return Err(("unexpected `.`".to_string(), res)),
            Some("(") => {
                let thing = parse(toki);
                let res2 = match thing {
                    Ok(x) => x,
                    Err((s, r)) => 
                        if s.as_slice() == "unexpected `)`" { r }
                        else { return Err((s.to_string(), r)) }
                };
                res2
            },
            Some(")") => return Err(("unexpected `)`".to_string(), res)),
            Some("T") => Lambda("x".to_string(), 0, box Lambda("y".to_string(), 0, box Variable("x".to_string(), 0))),
            Some("F") => Lambda("x".to_string(), 0, box Lambda("y".to_string(), 0, box Variable("y".to_string(), 0))),
            Some("S") => Lambda("w".to_string(), 0, box Lambda("y".to_string(), 0, box Lambda("x".to_string(), 0, box Call(box Variable("y".to_string(), 0), box Call(box Call(box Variable("w".to_string(), 0), box Variable("y".to_string(), 0)), box Variable("x".to_string(), 0)))))),
            Some("Z") => Lambda("x".to_string(), 0i16, box Call(box Call(box Call(box Variable("x".to_string(), 0i16), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("y".to_string(), 0i16)))), box Lambda("x".to_string(), 0i16, box Call(box Call(box Variable("x".to_string(), 0i16), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("y".to_string(), 0i16)))), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("x".to_string(), 0i16)))))), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("y".to_string(), 0i16))))),
            Some("*") => Lambda("x".to_string(), 0, box Lambda("y".to_string(), 0, box Lambda("z".to_string(), 0, box Call(box Variable("x".to_string(), 0), box Call(box Variable("y".to_string(), 0), box Variable("z".to_string(), 0)))))),
            Some("&") => Lambda("x".to_string(), 0, box Lambda("y".to_string(), 0, box Call(box Call(box Variable("x".to_string(), 0), box Variable("y".to_string(), 0)), box Lambda("x".to_string(), 0, box Lambda("y".to_string(), 0, box Variable("y".to_string(), 0)))))),
            Some("|") => Lambda("x".to_string(), 0, box Lambda("y".to_string(), 0, box Call(box Call(box Variable("x".to_string(), 0), box Lambda("x".to_string(), 0, box Lambda("y".to_string(), 0, box Variable("x".to_string(), 0)))), box Variable("y".to_string(), 0)))),
            Some("!") => Lambda("x".to_string(), 0i16, box Call(box Call(box Variable("x".to_string(), 0i16), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("y".to_string(), 0i16)))), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("x".to_string(), 0i16))))),
            Some("Y") => Lambda("y".to_string(), 0i16, box Call(box Lambda("x".to_string(), 0i16, box Call(box Variable("y".to_string(), 0i16), box Call(box Variable("x".to_string(), 0i16), box Variable("x".to_string(), 0i16)))), box Lambda("x".to_string(), 0i16, box Call(box Variable("y".to_string(), 0i16), box Call(box Variable("x".to_string(), 0i16), box Variable("x".to_string(), 0i16)))))),
            // XXX: remove when `P` works
            // Some("func") => Lambda("p".to_string(), 0i16, box Lambda("z".to_string(), 0i16, box Call(box Call(box Variable("z".to_string(), 0i16), box Call(box Lambda("w".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Lambda("x".to_string(), 0i16, box Call(box Variable("y".to_string(), 0i16), box Call(box Call(box Variable("w".to_string(), 0i16), box Variable("y".to_string(), 0i16)), box Variable("x".to_string(), 0i16)))))), box Call(box Variable("p".to_string(), 0i16), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("x".to_string(), 0i16)))))), box Call(box Variable("p".to_string(), 0i16), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("x".to_string(), 0i16))))))),
            // VERY STRANGE, does not work
            // Some("P") => Lambda("n".to_string(), 0i16, box Call(box Call(box Call(box Variable("n".to_string(), 0i16), box Lambda("p".to_string(), 0i16, box Lambda("l".to_string(), 0i16, box Call(box Call(box Variable("l".to_string(), 0i16), box Call(box Variable("p".to_string(), 0i16), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("y".to_string(), 0i16))))), box Call(box Lambda("w".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Lambda("x".to_string(), 0i16, box Call(box Variable("y".to_string(), 0i16), box Call(box Call(box Variable("w".to_string(), 0i16), box Variable("y".to_string(), 0i16)), box Variable("x".to_string(), 0i16)))))), box Call(box Variable("p".to_string(), 0i16), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("y".to_string(), 0i16))))))))), box Lambda("z".to_string(), 0i16, box Call(box Call(box Variable("z".to_string(), 0i16), box Lambda("s".to_string(), 0i16, box Lambda("z".to_string(), 0i16, box Variable("z".to_string(), 0i16)))), box Lambda("s".to_string(), 0i16, box Lambda("z".to_string(), 0i16, box Variable("z".to_string(), 0i16)))))), box Lambda("x".to_string(), 0i16, box Lambda("y".to_string(), 0i16, box Variable("y".to_string(), 0i16))))),
            Some(num) if from_str::<uint>(num.clone()).is_some() => {
                let nm = from_str::<int>(num.clone()).unwrap();
                let mut numb = Variable("z".to_string(), 0);
                for _ in range(0, nm) {
                    numb = Call(box Variable("s".to_string(), 0), box numb);
                }
                Lambda("s".to_string(), 0, box Lambda("z".to_string(), 0, box numb))
            },
            Some(name) => Variable(name.to_string(), 0),
            None => return Err(("".to_string(), res)),
        };
        res = if res == Nothing {
                  next
              } else {
                  Call(box res, box next)
              };
        tok = toki.next();
    }
    Ok(res)
}
