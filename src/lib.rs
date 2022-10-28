#![feature(box_patterns)]

pub mod instructor;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod types;

use std::time::Instant;

use tracing::{info, trace};
use types::{
    errors::{ZError, ZResult},
    printer::Print,
};

use crate::{
    instructor::gen_instructions,
    interpreter::interpret_asts,
    lexer::lex,
    parser::parse_token_list,
    types::{element::Element, interpreter_data::InterpreterData, typeobj::Type, value::Value},
};

pub fn compile(
    input: String,
    filename: &str,
    typelist: &mut InterpreterData<Type<Element>, impl Print>,
) -> ZResult<Vec<Element>> {
    /*if typelist.out.verbosity() == 0 {
        return gen_instructions(parse_token_list(lex(input, filename)?)?, typelist);
    }*/
    // TODO --stats flag

    info!("Lexing");
    let lex_start = Instant::now();
    let lexed = lex(input, filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    trace!("{lexed:#?}");

    info!("Parsing");
    let parse_start = Instant::now();
    let parsed = parse_token_list(lexed)?;
    let parse_time = parse_start.elapsed().as_micros();
    trace!("{parsed:#?}");

    info!("Generating instructions");
    let check_start = Instant::now();
    let instructions = gen_instructions(parsed, typelist)?;
    let check_time = check_start.elapsed().as_micros();
    trace!("{instructions:#?}");

    info!("Stats:");
    info!("Lexing time: {lex_time}µs");
    info!("Parsing time: {parse_time}µs");
    info!("Instruction generation time: {check_time}µs");
    info!("Total time: {}µs\n", lex_time + parse_time + check_time);

    Ok(instructions)
}

pub fn interpret(
    input: &Vec<Element>,
    i_data: &mut InterpreterData<Value, impl Print>,
) -> ZResult<i32> {
    /*if i_data.out.verbosity() == 0 {
        return interpret_asts(input, i_data);
    }*/
    // TODO --stats flag
    info!("Interpreting");
    let interpret_start = Instant::now();
    let exit_code = interpret_asts(input, i_data)?;
    let interpret_time = interpret_start.elapsed().as_micros();
    info!("Exited with code {exit_code}");
    info!("Stats");
    info!("Interpreting time: {interpret_time}µs");
    Ok(exit_code)
}
