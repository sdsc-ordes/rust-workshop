// This library crate creates a proc-macro `comp! [...]` which lets you write
// a python comprehension like `[ x*2 for x in [1, 2, 3, 4] ]`.
//
// The following Bakus Naur form describes the python comprehension:
//
// <list_comprehension> ::= "[" <mapping> <for_clause> <if_clause>* "]"
//
// <mapping>            ::= expression
// <for_clause>         ::= "for" <pattern> "in" <range>
// <if_clause>          ::= "if" expression
//
// <pattern>            ::= expression
// <range>              ::= expression

// In the beginning we need proc_macro2 and when we are finished, we
// turn these crate to `proc_macro` which is the compiler internal crate.
type TokenStream2 = proc_macro2::TokenStream;

use quote::quote;
use syn::{parse::ParseStream, parse_macro_input};

/// TODO: Implement here
/// - the types from the video
/// - the parsing of the types from the token streams
/// - and the quoting which turns the expressions back to Rust code.

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // TODO: Implement here the parsing and
    //       converting into a token stream again by "quoting".
    //       Also print the resulting token stream for inspection.

    eprintln!(
        "=======================\n\
        Proc-macro generated code:\n\
        '{}'\n\
        ========================",
        "not implemented"
    );

    input
}
