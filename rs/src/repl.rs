use std::io;
use std::io::Write;
use std::time::Instant;
use ansi_term::Color::{Cyan, Green, White, Yellow};
use text_io::read;
use crate::compile;
use crate::interpreter::interpret_expr;
use crate::objects::deferstack::DeferStack;
use crate::objects::typeobj::TypeObj;
use crate::objects::variable::Variable;
use crate::objects::stack::Stack;

pub fn repl(verbosity: u8) {
    let filename = "[stdin]".to_string();
    let mut typelist = Stack::<TypeObj>::default_type();
    let mut varlist = Stack::<Variable>::default_variable();
    let mut deferlist = DeferStack::new();
    let in_symbol = Cyan.bold().paint(">>]");
    let out_symbol = Green.bold().paint("[>>");
    println!("{}", Yellow.bold().paint(format!("Zyxt Repl (v{})", env!("CARGO_PKG_VERSION"))));
    println!("{}", Cyan.paint("`;exit` to exit"));
    loop {
        print!("{} ", in_symbol);
        io::stdout().flush().unwrap();
        let input: String = read!("{}\n");
        // TODO support for multiline

        if input == ";exit".to_string() {break;}
        let instructions = match compile(input, &filename, &mut typelist, verbosity) {
            Ok(v) => v,
            Err(e) => {e.print_noexit(); continue}
        };

        let instr_len = instructions.len();
        if verbosity >= 2 {println!("{}", Yellow.bold().paint("\nInterpreting"));}
        for (i, instr) in instructions.into_iter().enumerate() {
            match {
                if verbosity == 0 {interpret_expr(instr, &mut varlist, &mut deferlist)} else {
                    let interpret_start = Instant::now();
                    let result = interpret_expr(instr, &mut varlist, &mut deferlist);
                    let interpret_time = interpret_start.elapsed().as_micros();
                    println!("{}", White.dimmed().paint(format!("{}µs", interpret_time)));
                    result
                }} {
                Ok(result) => {
                    if result != Variable::Null && i == instr_len-1 {
                        println!("{} {}", out_symbol, Yellow.paint(result.to_string()))
                    }
                },
                Err(e) => { e.print_noexit(); }
            }
        }
    }
}