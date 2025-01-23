use crate::{
    logic::Logic,
    parser::{lexer::lex, test_parser},
    state::TypeSystem,
};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{extra::SimpleState, Parser as _};
use glob::glob;
use std::{fs::read_to_string, ops::Range};
use tracing::Level;
use yansi::Paint;

pub fn test(input_name: &Option<String>, level: &Level) {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(*level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    let files = if let Some(input_name) = input_name {
        vec![input_name.to_string()]
    } else {
        let mut files = vec![];
        for file in glob("**/*.type").unwrap() {
            files.push(file.unwrap().to_str().unwrap().to_string());
        }
        files
    };
    let mut results = vec![];
    for input_name in files {
        let input = read_to_string(&input_name).unwrap();
        let tokens = lex(&input);
        let (test, errors) = test_parser()
            .parse_with_state(tokens, &mut SimpleState::from(TypeSystem::new()))
            .into_output_errors();
        if !errors.is_empty() {
            println!("--- Failed to parse the input text for {} ---", input_name);
        }
        for error in &errors {
            print_error(&input, error, &input_name);
        }
        if !errors.is_empty() {
            println!("--- End of error ---");
        }
        if let Some(test) = test {
            let mut state = test.ts;
            let mut goals: Logic = test.goals.into();
            loop {
                let next = goals.reduce(&mut state, true);
                if next == goals {
                    break;
                }
                goals = next;
            }
            if goals == test.expected {
                results.push((input_name, true));
            } else {
                results.push((input_name, false));
            }
        } else {
            results.push((input_name, false));
        }
    }

    for test in &results {
        if test.1 {
            println!("{}: {}", test.0, "Passed".green());
        } else {
            println!("{}: {}", test.0, "Failed".red());
        }
    }

    let total = results.len();
    let passed = results.iter().filter(|(_, result)| *result).count();
    let failed = results.iter().filter(|(_, result)| !*result).count();
    println!(
        "{} tests passed, {} failed",
        format!("{} / {}", passed, total).green(),
        failed.to_string().red()
    )
}

pub fn print_error(
    input: &str,
    error: &chumsky::prelude::Rich<'_, crate::parser::lexer::Token>,
    input_name: &str,
) {
    let source = Source::from(input);
    let range: Range<usize> = (*error.span()).into();
    let mut report = Report::build(ReportKind::Error, (input_name, range.clone()));
    report.set_message("An error occurred while parsing the input text");
    report.add_label(
        Label::new((input_name, range.clone()))
            .with_message(error.to_string())
            .with_color(Color::Red),
    );
    report
        .finish()
        .eprint((input_name, source))
        .expect("Failed to print report");
}
