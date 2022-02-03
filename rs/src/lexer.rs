use regex::Regex;
use crate::syntax::lexing_old::{TokenEntry, TokenType, TokenCategory, token_catalogue};
use crate::{errors, Token};

#[derive(Clone)]
pub(crate) struct PositionTracker {
    pub(crate) filename: String,
    pub(crate) line: u32,
    pub(crate) column: u32,
    prev_column: u32,
    char_pos: u32
}
impl Default for PositionTracker {
    fn default() -> Self {
        PositionTracker {
            filename: String::from("[unknown]"),
            line: 1,
            column: 1,
            prev_column: 0,
            char_pos: 0
        }
    }
}

#[derive(Clone)]
pub struct StateTracker {
    pub(crate) position: PositionTracker,
    pub(crate) is_literal_string: bool,
    pub(crate) literal_string_type: TokenType,
    pub(crate) prev_type: TokenType,
    literal_string_line: u32,
    literal_string_column: u32,
    pub(crate) brackets: Vec<char>
}
impl Default for StateTracker {
    fn default() -> Self {
        StateTracker {
            position: PositionTracker::default(),
            is_literal_string: false,
            literal_string_type: TokenType::Null,
            prev_type: TokenType::Null,
            literal_string_line: 0,
            literal_string_column: 0,
            brackets: vec![]
        }
    }
}

fn get_next_char(c: char, input: &String, stack: &mut Vec<String>, states: &mut StateTracker) -> Result<char, bool> {
    if c == '\n' {
        states.position.line += 1;
        states.position.prev_column = states.position.column;
        states.position.column = 1;
    } else {states.position.column += 1;}
    states.position.char_pos += 1;
    match input.chars().nth(states.position.char_pos as usize) {
        Some(c) => {
            if (c == ' ' || c == '\n' || c == '\r') && states.is_literal_string {stack.push(String::from(c))}
            else if !(c == ' ' || c == '\n' || c == '\r') {stack.push(String::from(c))}
            Ok(c)
        }
        None => Err(false)
    }
}
fn get_next_char_noupdate(input: &String, states: &StateTracker) -> char {
    match input.chars().nth((states.position.char_pos + 1) as usize) {
        Some(c) => c,
        None => char::from(0)
    }
}

fn get_token_entry<'a>(stack: &Vec<String>, states: &'a StateTracker, input: &String) -> Option<(String, TokenEntry<'static>)> {
    for entry in token_catalogue().into_iter() {
        let value = entry.value;
        //while value.len() != 0 && value.chars().nth(value.len()-1).unwrap() == ' ' {value = &value[..value.len() - 1]};
        let re1 = Regex::new(&*entry.next_prohibited).unwrap();
        let re2 = Regex::new(&*entry.prohibited).unwrap();
        let joined_stack = stack.join("");
        let next_char = get_next_char_noupdate(input, states).to_string();

        if ((!entry.match_whole && joined_stack.ends_with(value))
            || (entry.match_whole && joined_stack == value)) // if the stack ends with the token tested
           && (entry.condition)(states) // and the stack satisfies the conditions
           && (entry.next_prohibited.len() == 0
            || re1.is_match(&*next_char)) // and the next character is invalid to be part of the token
           && ((entry.prohibited.len() == 0 // and the stack itself is valid
            || !re2.is_match(&*joined_stack)) && joined_stack.len() != 0) {
            return if value.len() == 0 {Some((joined_stack, entry))}
                else {Some((String::from(value), entry))}
        }
    }
    None
}

pub fn lex(preinput: String, filename: &String) -> Vec<Token> {
    if preinput.trim().len() == 0 {return Vec::new()};
    let input = preinput + "\n";
    let mut out: Vec<Token> = vec![];
    let mut stack: Vec<String> = vec![];

    let mut states = StateTracker{
        position: PositionTracker {
            filename: filename.clone(),
            ..Default::default()
        },
        ..Default::default()
    };
    let mut c = input.chars().nth(0).unwrap();
    stack.push(String::from(c));

    'main: loop {
        if c == '\r' && !states.is_literal_string {
            if let Ok(nc) = get_next_char(c, &input, &mut stack, &mut states) {c = nc;}
            else {break 'main};
            continue
        }
        if let Some((token, token_entry)) = get_token_entry(&stack, &states, &input) {
            if token_entry.categories.contains(&TokenCategory::LiteralStringEnd) {
                out.push(Token {
                    value: String::from(&stack.join("")[0..stack.len() - token.len()]),
                    type_: states.literal_string_type,
                    line: states.literal_string_line,
                    column: states.literal_string_column,
                    categories: &[TokenCategory::Literal]
                });
                stack.clear();
                stack.append(&mut Vec::from_iter(token.split("").map(|s| s.to_string())));
                states.literal_string_line = 0;
                states.literal_string_column = 0;
            } else if token_entry.categories.contains(&TokenCategory::LiteralStringStart) {
                states.literal_string_line = states.position.line;
                states.literal_string_column = states.position.column + 1;
            }

            states = (token_entry.state_changes)(&mut states);
            states.prev_type = token_entry.type_;

            out.push(Token{
                value: stack.join(""),
                type_: token_entry.type_,
                line: states.position.line,
                column: states.position.column + 1 - token.len() as u32,
                categories: token_entry.categories
            });
            stack.clear();
        }

        if let Ok(nc) = get_next_char(c, &input, &mut stack, &mut states) {c = nc;}
        else {break 'main;}
    }

    if stack.join("").trim().len() != 0 {
        out.push(Token {
            value: stack.join(""),
            type_: TokenType::Variable,
            line: states.position.line,
            column: states.position.column + 1 - stack.join("").trim().len() as u32,
            categories: &[]
        })
    }

    let mut cursor = 0;
    let mut selected: &Token;
    let mut new_out = vec![];
    while cursor < out.len() {
        selected = &out[cursor];
        if selected.type_ == TokenType::DotOpr
            && (cursor != 0 && out.get(cursor-1).unwrap().type_ == TokenType::LiteralNumber)
            && (cursor != out.len() - 1 && out.get(cursor+1).unwrap().type_ == TokenType::LiteralNumber) {
            new_out.pop();
            new_out.push(Token {
                value: format!("{}.{}", &*out.get(cursor - 1).unwrap().value, &*out.get(cursor + 1).unwrap().value),
                type_: TokenType::LiteralNumber,
                line: out.get(cursor - 1).unwrap().line,
                column: out.get(cursor - 1).unwrap().column,
                categories: &[TokenCategory::Literal]
            });
            cursor += 1;
        } else {new_out.push(out[cursor].clone())}
        cursor += 1
    }

    if states.brackets.len() != 0 {
        errors::error_pos(filename, states.position.line, states.position.column);
        errors::error_2_0_1(*states.brackets.last().unwrap())
    }

    new_out
}