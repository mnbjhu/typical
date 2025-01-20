use crate::{
    parser::{lexer::lex, test::test_parser},
    state::TypeSystem,
};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{extra::SimpleState, Parser};
use std::{fs::read_to_string, ops::Range};

pub fn test(input_name: String) {
    let input = read_to_string(&input_name).unwrap();
    let tokens = lex(&input);
    let (ast, errors) = test_parser()
        .parse_with_state(tokens, &mut SimpleState::from(TypeSystem::new()))
        .into_output_errors();
    for error in errors {
        let source = Source::from(&input);
        let range: Range<usize> = (*error.span()).into();
        let mut report = Report::build(ReportKind::Error, (&input_name, range.clone()));
        report.set_message("An error occurred while parsing the input text");
        report.add_label(
            Label::new((&input_name, range.clone()))
                .with_message(error.to_string())
                .with_color(Color::Red),
        );
        report
            .finish()
            .eprint((&input_name, source))
            .expect("Failed to print report");
    }

    println!("{:#?}", ast);
}
