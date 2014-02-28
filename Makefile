all: lambda main

lambda:
	rustc lambda.rs

main:
	rustc -L . main.rs

.PHONY: main lambda
