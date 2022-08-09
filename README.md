### A simple calculator written in Rust.

This is a fist time I learn about parser and Rust, so I decide to write a simple calculator using the **Recursive Descent** algorithm with the following grammar:

```
E --> T {( "+" | "-" ) T}
T --> F {( "*" | "/" ) F}
F --> P ["^" F]
P --> v | "(" E ")" | "-" T | Function "(" {E[,]} ")"
```

This grammar is base on this awesome [guide](https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm). I also extends it to support function.

#### Example:

```
cargo run "1+2*sqrt(4)"
```

#### Feature:

- Support the following operators: `+`,`-`,`*`,`/`,`^`.
- Built in functions:
  - [x] `sin`
  - [x] `cos`
  - [x] `tan`
  - [x] `sqrt`
  - [x] `abs`
  - [x] `max`: `max(arg1,arg2,...)`
  - [x] `min`: `min(arg1,arg2,...)`
- Built in constants:
  - [x] `PI`
  - [x] `e`

#### TODO:

- Support for variables.
- Better error handling.
