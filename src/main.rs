mod arguments;
mod mining;
mod simulations;
mod techniques;

fn main() {
    
    arguments::handle();
}

#[derive(Clone)]
pub enum Verbosity {
    Low,
    High,
    None,
}

