use backtrace::Backtrace;
use std::process::exit;
use ansi_term::Color::{Black, Red, Yellow};
use ansi_term::Style;
use crate::{Element, Token, Type};
use crate::objects::variable::Variable;
use crate::objects::position::Position;
use crate::objects::token::Keyword;

#[derive(Clone)]
pub struct ZyxtError {
    pub position: Vec<(Position, String)>,
    pub code: &'static str,
    pub message: String
}
impl ZyxtError {
    pub fn print(&self) -> ! {
        self.print_noexit();
        exit(0)
    }
    pub fn print_noexit(&self) {
        for (pos, raw) in &self.position {
            println!("{}{}", Style::new().on(Red).bold().paint(
                format!(" {} ", pos)
            ), Style::new().bold().paint(
                format!(" {} ", raw)
            ));
        }
        println!("{}", Black.on(Yellow).paint(format!(" Error {} ", self.code)).to_string()
            + &*Red.bold().paint(format!(" {}", self.message)).to_string());
    }
    pub fn from_pos_and_raw(pos: &Position, raw: &String) -> PositionForZyxtError {
        PositionForZyxtError {position: vec![(pos.clone(), raw.clone().trim().to_string())]}
    }
    pub fn from_element(element: &Element) -> PositionForZyxtError {
        PositionForZyxtError {position: vec![(element.get_pos().clone(), element.get_raw().trim().to_string())]}
    }
    pub fn from_token(token: &Token) -> PositionForZyxtError {
        PositionForZyxtError {position: vec![(token.position.clone(), token.value.clone().trim().to_string())]}
    }
    pub fn no_pos() -> PositionForZyxtError {
        PositionForZyxtError {position: vec![]}
    }
}
pub struct PositionForZyxtError {
    position: Vec<(Position, String)>
}
#[allow(dead_code)]
impl PositionForZyxtError {
    // TODO call stack thing
    /* 0. Internal errors, have to do with the compiler-interpreter itself */
    /// Rust error
    pub fn error_0_0(self, error: String, backtrace: Backtrace) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "0.0",
            message: format!("Internal error: \n{}\n{:?}\n\nThis shouldn't happen! Open an issue on our Github repo page: [TODO]", error, backtrace)
        }
    }

    /// No file given
    pub fn error_0_1(self) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "0.1",
            message: "No file given".to_string()
        }
    }

    /* 1. File and I/O errors */
    /// File does not exist
    pub fn error_1_0(self, filename: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "1.0",
            message: format!("File `{}` does not exist", filename)
        }
    }

    /// file cannot be opened
    pub fn error_1_1(self, filename: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "1.1",
            message: format!("File `{}` cannot be opened", filename)
        }
    }

    pub fn error_1_2(self, dirname: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "1.2",
            message: format!("Directory given (Got `{}`)", dirname)
        }
    }

    /* 2. Syntax errors */
    /// parentheses not closed properly (try swapping)
    pub fn error_2_0_0(self, paren1: String, paren2: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.0.0",
            message: format!("Parentheses `{}` and `{}` not closed properly; try swapping them", paren1, paren2)
        }
    }
    /// parentheses not closed properly (not closed)
    pub fn error_2_0_1(self, paren: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.0.1",
            message: format!("Parenthesis `{}` not closed", paren)
        }
    }
    /// parentheses not closed properly (not opened)
    pub fn error_2_0_2(self, paren: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.0.2",
            message: format!("Parenthesis `{}` not opened", paren)
        }
    }

    /// unexpected ident (generic)
    pub fn error_2_1_0(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.0",
            message: format!("Unexpected ident `{}`", ident)
        }
    }
    /// unexpected ident (lexer didnt recognise)
    pub fn error_2_1_1(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.1",
            message: format!("Ident `{}` not recognised by lexer", ident)
        }
    }
    /// unexpected ident (dot at end of expression)
    pub fn error_2_1_2(self) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.2",
            message: "Stray `.` at end of expression".to_string()
        }
    }
    /// unexpected ident (binary operator at start/end of expression)
    pub fn error_2_1_3(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.3",
            message: format!("Stray `{}` binary operator at start/end of expression", ident)
        }
    }
    /// unexpected ident (unary operator at start/end of expression)
    pub fn error_2_1_4(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.4",
            message: format!("Stray `{}` unary operator at start/end of expression", ident)
        }
    }
    /// unexpected ident (declaration expr at start/end of expression)
    pub fn error_2_1_5(self) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.5",
            message: "Stray `:=` at start/end of expression".to_string()
        }
    }
    /// unexpected ident (non-flag between first flag and declared variable)
    pub fn error_2_1_6(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.6",
            message: format!("Stray `{}` between first flag and declared variable", ident)
        }
    }
    /// unexpected ident ('else/elif'  found after 'else' keyword)
    pub fn error_2_1_7(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.7",
            message: format!("`{}` detected after `else` keyword", ident)
        }
    }
    /// unexpected ident (block expected, not ident)
    pub fn error_2_1_8(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.8",
            message: format!("Block expected, not `{}`", ident)
        }
    }
    /// unexpected ident ('else/elif' found without 'if' keyword)
    pub fn error_2_1_9(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.9",
            message: format!("Stray `{}` without starting `if`", ident)
        }
    }
    /// unexpected ident (stray comment start / end)
    pub fn error_2_1_10(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.10",
            message: format!("Stray unclosed/unopened `{}`", ident)
        }
    }
    /// unexpected ident (must be variable)
    pub fn error_2_1_11(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.11",
            message: format!("Only variables can be deleted (Got `{}`)", ident)
        }
    }
    /// unexpected ident (cannot delete dereferenced variable)
    pub fn error_2_1_12(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.12",
            message: format!("Cannot delete dereferenced variable (Got `{}`)", ident)
        }
    }
    /// unexpected ident (bar not closed)
    pub fn error_2_1_13(self) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.13",
            message: "Opening bar not closed".to_string()
        }
    }
    /// unexpected ident (Extra values past default value)
    pub fn error_2_1_14(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.14",
            message: format!("Extra values past default value (Got `{}`)", ident)
        }
    }
    /// unexpected ident (Variable name isn't variable)
    pub fn error_2_1_15(self, ident: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.15",
            message: format!("Variable name isn't variable (Got `{}`)", ident)
        }
    }
    /// unexpected ident (pre keyword at end of expression)
    pub fn error_2_1_16(self) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.16",
            message: "`pre` at end of line".to_string()
        }
    }
    /// unexpected ident (parameters with class keyword)
    pub fn error_2_1_17(self) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.17",
            message: "Parameters found after `class` keyword".to_string()
        }
    }
    /// unexpected ident (parameters with class keyword)
    pub fn error_2_1_18(self, kwd: &Keyword) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.1.18",
            message: format!("Block expected after `{:?}`", kwd)
        }
    }

    /// assignment without variable name
    pub fn error_2_2(self) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.2",
            message: "Assignment without variable name".to_string()
        }
    }

    /// unfilled argument
    pub fn error_2_3(self, arg: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "2.3",
            message: format!("Unfilled argument `{}`", arg)
        }
    }

    /* 3. Variable & attribute errors */
    /// Variable not defined
    pub fn error_3_0(self, varname: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "3.0",
            message: format!("Undefined variable `{}`", varname)
        }
    }

    /// Type has no attribute (typechecker)
    pub fn error_3_1_0(self, parent: Element, parent_type: Type, attribute: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "3.1.0",
            message: format!("`{}` (type `{}`) has no attribute `{}`", parent.get_raw().trim(), parent_type, attribute)
        }
    }
    /// Type has no attribute (interpreter)
    pub fn error_3_1_1(self, parent: Variable, attribute: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "3.1.1",
            message: format!("`{}` (type `{}`) has no attribute `{}`", parent, parent.get_type_obj(), attribute)
        }
    }

    /* 4. Type errors */
    /// Binary operator not implemented for type
    pub fn error_4_0_0(self, operator: String, type1: String, type2: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "4.0.0",
            message: format!("Operator {} not implemented for types `{}`, `{}`", operator, type1, type2)
        }
    }
    /// Unary operator not implemented for type
    pub fn error_4_0_1(self, operator: String, type_: String) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "4.0.1",
            message: format!("Operator {} not implemented for type `{}`", operator, type_)
        }
    }

    /// Binary operation unsuccessful
    pub fn error_4_1_0(self, operator: String, value1: Variable, value2: Variable) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "4.1.0",
            message: format!("Operator {} unsuccessful on `{}` (type `{}`), `{}` (type `{}`)",
                             operator, value1, value1.get_type_obj(), value2, value2.get_type_obj())
        }
    }
    /// Unary operation unsuccessful
    pub fn error_4_1_1(self, operator: String, value: Variable) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "4.1.1",
            message: format!("Operator {} unsuccessful on `{}` (type `{}`)",
                             operator, value, value.get_type_obj())
        }
    }

    /// Non-i32 script return value
    pub fn error_4_2(self, value: Variable) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "4.2",
            message: format!("Non-i32 script return value detected (Got `{}`)", value)
        }
    }

    /// Wrong type assigned to variable
    pub fn error_4_3(self, variable: String, var_type: Type, value_type: Type) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "4.3",
            message: format!("Value of type `{}` assigned to variable `{}` of type `{}`",
                value_type, variable, var_type)
        }
    }

    /// inconsistent block return type (temporary)
    pub fn error_4_t(self, block_type: Type, return_type: Type) -> ZyxtError {
        ZyxtError {
            position: self.position,
            code: "4.4",
            message: format!("Block returns variable of type `{}` earlier on, but also returns variable of type `{}`", block_type, return_type)
        }
    }
}