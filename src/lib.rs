//! Sample Molt Extension

use molt::Interp;
use molt::MoltResult;
use molt::Value;
use molt::molt_ok;
use molt::check_args;
use molt::ResultCode;

/// Install the extension's commands into the Interp.
pub fn install(interp: &mut Interp) -> MoltResult {
    // Install a command implemented in Rust.
    interp.add_command("double", cmd_double);

    // NEXT, load the extension's Tcl code
    if let Err(ResultCode::Error(value)) = interp.eval(include_str!("lib.tcl")) {
        panic!("Error in benchmark Tcl library: {}", value.as_str());
    }

    molt_ok!()
}

/// # double *x*
///
/// Computes the double of a value
fn cmd_double(_interp: &mut Interp, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 2, 2, "x")?;

    // Get x, if it's an integer
    let x = argv[1].as_int()?;

    // Return the result.
    molt_ok!(2 * x)
}
