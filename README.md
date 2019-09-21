# molt-sample: A sample Molt REPL application

This repo shows how to create a basic [Molt](https://github.com/wduquette/molt) shell
application using the `molt` and `molt-shell` crates, including how to define a simple
Molt command, `square`.  For more information, see the rustdocs and
[The Molt Book](https://wduquette.github.io/molt).

Feel free to fork this repo as the basis for new Molt shell applications.

The following assume that Rust and Cargo are installed.

To run an interactive REPL:

```sh
$ cargo run
   Compiling molt-sample v0.1.0 (/Users/will/github/molt-sample)
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
   Compiling molt-sample v0.1.0 (/Users/will/github/molt-sample)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/molt-sample sample.tcl`
In sample.tcl
The square of 3 is 9.
$
```

To run the script from inside the REPL:

```sh
$ cargo run
   Compiling molt-sample v0.1.0 (/Users/will/github/molt-sample)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/molt-sample`
% source sample.tcl
In sample.tcl
The square of 3 is 9.
%
```
