use molt::check_args;
use molt::molt_ok;
use molt::Interp;
use molt::MoltResult;
use molt::Value;

fn main() {
    use std::env;

    // FIRST, get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // NEXT, create and initialize the interpreter.
    let mut interp = Interp::new();

    // NOTE: commands can be added to the interpreter here.

    // Add a command defined in this file.
    interp.add_command("square", cmd_square);

    // Install a Molt extension crate
    molt_sample::install(&mut interp).expect("Could not install.");

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
