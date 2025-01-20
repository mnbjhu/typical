pub mod lex;
pub mod test;

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Test a type file
    Test {
        /// The file to test
        file: String,
    },

    /// Lex a file
    Lex {
        /// The file to lex
        file: String,
    },
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Test { file } => test::test(file.to_string()),
            Command::Lex { file } => lex::lex(file.to_string()),
        }
    }
}
