# molt-sample: A sample Molt extension

This repo shows how to create a basic [Molt](https://github.com/wduquette/molt) shell
application using the `molt` and `molt-shell` crates, including how to extend the shell
with Molt commands and how to define Molt extension crates.  For more information, see the
rustdocs and [The Molt Book](https://wduquette.github.io/molt).

Feel free to fork this repo as the basis for new Molt shell applications and extension crates.

## Structure

The file `src/main.rs` creates a Molt interpreter and extends it with a command, `square`, that
computes the square of an integer.  

It also installs the `molt_sample::` extension crate, which is defined in `src/lib.rs`. The
sample extension defines two Molt commands.  `double` doubles an integer value, and is defined
in Rust; `triple` triples an integer value, and is defined in `src/lib.tcl`.

Thus, this sample can be used as a standalone application; but it can also be used a dependency
for another Rust project and its commands installed into that project's interpreter.

## Running the REPL

The following assume that Rust and Cargo are installed.

To run an interactive REPL:

```sh
$ cargo run
   Compiling molt-sample v0.3.0 (/Users/will/github/molt-sample)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/molt-sample`
% square 2
4
% exit
$
```

To run a script:

```sh
$ cargo run sample.tcl
   Compiling molt-sample v0.3.0 (/Users/will/github/molt-sample)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/molt-sample sample.tcl`
In sample.tcl
The square of 3 is 9.
$
```

To run the script from inside the REPL:

```sh
$ cargo run
   Compiling molt-sample v0.3.0 (/Users/will/github/molt-sample)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/molt-sample`
% source sample.tcl
In sample.tcl
The square of 3 is 9.
%
```
