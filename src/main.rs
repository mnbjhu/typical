use clap::Parser as _;
use cli::Command;

mod cli;
mod logic;
mod parser;
mod state;
mod ty;

fn main() {
    tracing_subscriber::fmt::init();
    Command::parse().run();
}
