use tracing::Level;

pub mod lex;
pub mod repl;
pub mod test;

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Execute tests in the current directory or a specific file
    Test {
        /// File to test. If not provided, the current directory is used
        #[clap(short, long)]
        file: Option<String>,

        /// The minimum log level. If not provided, the default is `error`
        #[clap(short, long, default_value = "error")]
        log_level: Level,
    },

    /// Start the REPL
    Repl,

    /// Lex a file
    Lex {
        /// The file to lex
        file: String,
    },
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Test { file, log_level } => test::test(file, log_level),
            Command::Lex { file } => lex::lex(file.to_string()),
            Command::Repl => repl::repl().unwrap(),
        }
    }
}
