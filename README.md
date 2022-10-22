# Description

This repository holds simple, but annotated example intended to understand concepts about the Rust programming language.

To test an example, pass the name of the function to cargo as follows:

```Bash
cargo run -- example_name example_arg1, example_arg2, example_arg2, ...
```

where `example_name` is the name of the function to test, `example_arg1` is the first argument of the function, `example_arg2` is the second argument of the function and so on. These arguments depends on the function to be executed.

## hello

This example prints in the terminal a greeting passed via the command line to the function `hello`. Then this function reads from the standard input a name. Finally, the function *greets* the user that inputs their name and exits.