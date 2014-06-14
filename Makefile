all: lambda main doc

lambda: liblambda-*

liblambda-*: lambda.rs parse.rs
	rustc lambda.rs

main: liblambda-* main.rs
	rustc -L . main.rs

test: lambda.rs parse.rs test.rs
	rustc -L . lambda.rs --cfg test && ./test

doc: liblambda-*
	rustdoc lambda.rs

.PHONY: lambda
