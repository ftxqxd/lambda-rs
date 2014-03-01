lambda-rs
=========

lambda-rs is a basic parsing/reducing library for the λ-calculus.

It also contains a basic command-line interface.

Build
-----

    make # Compile lambda.rs and main.rs, then build docs
    make test

Syntax
------

The syntax for expressions, as parsed by `from_str`, is as follows:

```
expr ::= <lambda> | <IDENT> | <call> | '(' <expr> ')'
lambda ::= ('\\' | 'λ') <IDENT>+ '.' <expr>
call ::= <expr> <expr>
```

Some extra constants are also provided:

* Numbers (`1`, `2`, `3`, ...)
* Booleans (`T`, `F`)
* Successor function (`S`)
* Zero conditional (`Z`)
* Multiplication (`*`)
* Logical operators (`&`, `|`, `!`)
* [Fixed-point combinator](https://en.wikipedia.org/wiki/Fixed-point_combinator) (`Y`)
