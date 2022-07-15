### A simple calculator written in Rust.

This is a fist time I learn about parser and Rust, so I decide to write a simple calculator using the **Recursive Descent** algorithm with the following grammar:

```
E --> T {( "+" | "-" ) T}
T --> F {( "*" | "/" ) F}
F --> P ["^" F]
P --> v | "(" E ")" | "-" T | Function "(" {E[,]} ")"
```

This grammar is base on this awesome [guide](https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm). I also extends it to support function.

#### Feature:

- Support the following operators: `+`,`-`,`*`,`/`,`^`.
- Built in functions: `sin`, `cos`.

#### TODO:

- Support for variables.
- Support for colapse variables: `x + x => 2*x`
- Support for resolving basic equation.
- Better error handling.
