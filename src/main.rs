use clap::Parser as _;
use cli::Command;

mod cli;
mod logic;
mod parser;
mod state;
mod ty;

fn main() {
    Command::parse().run();
}
