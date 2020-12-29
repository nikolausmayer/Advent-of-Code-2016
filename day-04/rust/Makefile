##
#
# Author: Nikolaus Mayer, 2020 (mayern@cs.uni-freiburg.de)
#
##

## Compilers
RUST = rustc

## Every *.rs file is a Rust source file
SOURCES = $(wildcard *.rs)

## Derive an "app" name for every "app.rs" source file
TARGETS = $(basename $(SOURCES))


## Tell the 'make' program that e.g. 'make clean' is not supposed to 
## create a file 'clean'
##
## "Why is it called 'phony'?" -- because it's not a real target. That 
## is, the target name isn't a file that is produced by the commands 
## of that target.
.PHONY: all clean 

## An empty suffixes rules disables built-in recipes, like "--no-builtin-rules"
.SUFFIXES:


## Default is release build mode
all: $(TARGETS)

## Remove built object files and the main executable
clean:
	$(info ... deleting built object files and executable  ...)
	-rm $(TARGETS)

## A main executable depends on its own source file 
%: %.rs Makefile
	$(info ... compiling $@ ...)
	$(RUST) $< -o $@

