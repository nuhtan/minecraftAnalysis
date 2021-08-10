mod arguments;
mod mining;
mod simulations;
mod techniques;

fn main() {
    
    arguments::handle();
}

/// The level of verboseness that the output from the program will correspond with.
///
/// * `Low` - Corresponds with printing lines corresponding to the current progress in a simulations.
/// * `High` - Does everything that `Low` does with extra details and information about how long sections took.
/// * `None` - Prints nothing extra.
#[derive(Clone, PartialEq)]
pub enum Verbosity {
    Low,
    High,
    None,
}

