use std::io;

mod cli;
mod tui;
fn main() -> io::Result<()> {
    cli::JamnCli::parse_and_dispatch()
}
