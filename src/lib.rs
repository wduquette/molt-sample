//! Sample Molt Extension

use molt::check_args;
use molt::molt_ok;
use molt::Interp;
use molt::types::*;

/// Install the extension's commands into the Interp.
pub fn install(interp: &mut Interp) -> MoltResult {
    // Install a command implemented in Rust.
    interp.add_command("double", cmd_double);

    // NEXT, load the extension's Tcl code
    if let Err(exception) = interp.eval(include_str!("lib.tcl")) {
        panic!("Error in benchmark Tcl library: {}", exception.value().as_str());
    }

    molt_ok!()
}

/// # double *x*
///
/// Computes the double of a value
fn cmd_double(_interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 2, 2, "x")?;

    // Get x, if it's an integer
    let x = argv[1].as_int()?;

    // Return the result.
    molt_ok!(2 * x)
}
