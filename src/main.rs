use molt::check_args;
use molt::MoltResult;
use molt::Value;
use molt::Interp;
use molt::molt_ok;

fn main() {
    use std::env;

    // FIRST, get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // NEXT, create and initialize the interpreter.
    let mut interp = Interp::new();

    // NOTE: commands can be added to the interpreter here.
    interp.add_command("square", cmd_square);

    // NEXT, evaluate the file, if any.
    if args.len() > 1 {
        molt_shell::script(&mut interp, &args[1..]);
    } else {
        molt_shell::repl(&mut interp, "% ");
    }
}

/// # square *x*
///
/// Computes the square of a value
pub fn cmd_square(_interp: &mut Interp, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 2, 2, "x")?;

    // Get x, if it's an integer
    let x = argv[1].as_int()?;

    // Return the result.
    molt_ok!(x * x)
}
