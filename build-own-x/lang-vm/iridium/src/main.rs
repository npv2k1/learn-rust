pub mod instruction;
pub mod vm;

pub mod repl;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
