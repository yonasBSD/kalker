use std::{env, process};

mod interpreter;
mod lexer;
mod parser;
mod prelude;
mod symbol_table;
mod visitor;
use parser::{ParserContext, Unit};

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[allow(unused_assignments)] // The compiler gives a warning that is not valid.
fn main() {
    let mut parser = ParserContext::new();

    // Command line argument input, execute it and exit.
    if let Some(expr) = env::args().skip(1).next() {
        eval(&mut parser, &expr);
        return;
    }

    // REPL
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                eval_repl(&mut parser, &input);
            }
            Err(ReadlineError::Interrupted) => break,
            _ => break,
        }
    }
}

fn eval_repl(parser: &mut ParserContext, input: &str) {
    match input {
        "" => eprint!(""),
        "clear" => print!("\x1B[2J"),
        "exit" => process::exit(0),
        _ => eval(parser, input),
    }
}

fn eval(parser: &mut ParserContext, input: &str) {
    match parser::parse(parser, input, get_angle_unit()) {
        Ok(result) => println!("{}", result),
        Err(_) => println!("Invalid expression"),
    }
}

fn get_angle_unit() -> Unit {
    if let Ok(angle_unit_var) = env::var("ANGLE_UNIT") {
        match angle_unit_var.as_ref() {
            "radians" => Unit::Radians,
            "degrees" => Unit::Degrees,
            _ => {
                panic!("Unexpected angle unit: {}.", angle_unit_var);
            }
        }
    } else {
        Unit::Radians
    }
}
