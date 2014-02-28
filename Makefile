all: lambda main

lambda:
	rustc lambda.rs

main:
	rustc -L . main.rs

test:
	rustc -L . test.rs --cfg test && ./test

.PHONY: main lambda test
