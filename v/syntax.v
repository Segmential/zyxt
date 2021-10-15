pub enum TokenType {
    comment_start // //
    comment_end // \n
    multiline_comment_start // /*
    multiline_comment_end // */
    flag // hoi, pub, priv, prot, const
    unary_opr // b~, ++, ! etc
    assignment_opr // =, +=, etc
    arithmetic_bitwise_opr // +, -, /f, rt, b& etc
    relational_opr // ==, >, is etc
    logical_opr // &&, ||, ^^ etc
    concat_opr // ..
    swap_opr // ><
    type_opr // istype, isnttype et
    literal // 3, true, null, etc
	literal_string // "abc" etc
    statement_end // ;
    comment
    variable
	null
}

// TODO make functions for literal_strings
struct TokenEntry {
    type_ TokenType // the type of the token
	condition fn (&StateTracker) bool = fn (states &StateTracker) bool {
		return !states.is_literal_string
	} // conditions needed for the token to be valid; do not change states.prev_type that is already handled by the lexer
	state_changes fn (&StateTracker) = fn(states &StateTracker) {} // the state changes that are taken place after the token is validated
	prohibited string // the values for the token to be invalid, given as a regex (if the token is a "")
	next_prohibited string // the values for the next character that are invalid, given as a regex of a single character
	is_literal_string_start bool // if the token marks the start of a literal string.
	is_literal_string_end bool // if the token marks the end of a literal_string.
}

const token_catalogue = {
    "//": TokenEntry{
		type_: .comment_start
		condition: fn (states &StateTracker) bool {
			return states.prev_type != .comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = true
			states.literal_string_type = .comment
		}
		is_literal_string_start: true
	}
	"\n": TokenEntry{
		type_: .comment_end
		condition: fn (states &StateTracker) bool {
			return states.prev_type == .comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = false
			states.literal_string_type = .null
		}
		is_literal_string_end: true
	}
	"/*": TokenEntry{
		type_: .multiline_comment_start
		condition: fn (states &StateTracker) bool {
			return states.prev_type != .multiline_comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = true
			states.literal_string_type = .comment
		}
		is_literal_string_start: true
	}
	"*/": TokenEntry{
		type_: .multiline_comment_end
		condition: fn (states &StateTracker) bool {
			return states.prev_type == .multiline_comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = false
			states.literal_string_type = .null
		}
		is_literal_string_end: true
	}
	"=": TokenEntry{
		type_: .assignment_opr
		next_prohibited: r"="
	}
	"hoi": TokenEntry{
		type_: .flag
		next_prohibited: r"\S"
	}
	"pub": TokenEntry{
		type_: .flag
		next_prohibited: r"\S"
	}
	"priv": TokenEntry{
		type_: .flag
		next_prohibited: r"\S"
	}
	"prot": TokenEntry{
		type_: .flag
		next_prohibited: r"\S"
	}
	"const": TokenEntry{
		type_: .flag
		next_prohibited: r"\S"
	}
	"": TokenEntry{
		type_: .variable
		prohibited: r"\W"
		next_prohibited: r"[\W\s]"
	}
}