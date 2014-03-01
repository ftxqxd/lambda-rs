all: lambda main doc

lambda:
	rm liblambda-*
	rustc lambda.rs

main:
	rustc -L . main.rs

test:
	rustc -L . test.rs --cfg test && ./test

doc:
	rustdoc lambda.rs

.PHONY: main lambda test doc
