use std::io::stdout;

use chumsky::{extra::SimpleState, Parser};
use pretty::BoxAllocator;
use rustyline::{error::ReadlineError, DefaultEditor, Result};

use crate::{
    parser::{
        lexer::lex,
        repl::{repl_parser, ListItem, ReplCommand},
    },
    state::TypeSystem,
};

use super::test::print_error;

pub fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut state = SimpleState::from(TypeSystem::new());
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let input = lex(&line);
                let (output, errors) = repl_parser()
                    .parse_with_state(input, &mut state)
                    .into_output_errors();
                for error in &errors {
                    print_error(&line, error, "<input>");
                }
                if let Some(output) = output {
                    output.handle(&mut state);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Bye!");
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

impl ReplCommand {
    pub fn handle(self, state: &mut TypeSystem) {
        match self {
            ReplCommand::Goal(goal) => {
                state.add_goal(goal);
                state
                    .goal
                    .pretty::<_, ()>(&BoxAllocator)
                    .render(60, &mut stdout())
                    .unwrap();
                println!();
            }
            ReplCommand::Eval(logic) => {
                let mut res = String::new();
                logic
                    .reduce(state, false)
                    .pretty::<_, ()>(&BoxAllocator)
                    .render_fmt(60, &mut res)
                    .unwrap();
                println!("{}", res);
            }
            ReplCommand::Impl => println!("Ok"),
            ReplCommand::Decl => println!("Ok"),
            ReplCommand::List(list_item) => match list_item {
                ListItem::Decl => state.decls.values().for_each(|decl| println!("{decl}")),
                ListItem::Impl(ident) => {
                    if ident.is_some() {
                        todo!()
                    } else {
                        state.impls.iter().for_each(|impl_| {
                            println!("{}", impl_);
                        });
                    }
                }
                ListItem::Vars => {
                    state.type_vars.iter().for_each(|(id, ty)| {
                        println!("${}: {}", id, ty);
                    });
                }
            },
            ReplCommand::New(ty) => println!("{}", ty.inst(state)),
            ReplCommand::Resolve(ty) => println!("{}", ty.resolve(state)),
        }
    }
}
