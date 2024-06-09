use cesm::command::{add_js::run_add_js, CliArgs, CliCommand};
use clap::Parser;

#[doc(hidden)]
fn main() {
    let args = CliArgs::parse();
    match args.cmd {
        Some(CliCommand::AddJs(run_args)) => {
            if let Err(err) = run_add_js(&run_args.input) {
                eprintln!("\x1b[91mError: {}\x1b[0m", err);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("\x1b[91mCommand not found\x1b[0m");
        }
    }
}
