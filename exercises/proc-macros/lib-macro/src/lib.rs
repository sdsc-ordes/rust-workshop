// This library crate creates a proc-macro `py_comp! [...]` which lets you write
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

use syn::parse::ParseStream;
use syn::Result;

#[derive(Debug)]
struct Comprehension {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

#[derive(Debug)]
struct Mapping(syn::Expr);

#[derive(Debug)]
struct ForIfClause {
    pattern: Pattern,
    range: syn::Expr,
    condition: Conditions,
}

#[derive(Debug)]
struct Pattern(syn::Pat);

#[derive(Debug)]
struct Condition(syn::Expr);

#[derive(Debug)]
struct Conditions(Vec<Condition>);

impl syn::parse::Parse for Pattern {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::Pat::parse_single(input).map(Self)
    }
}

impl syn::parse::Parse for Condition {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the 'if' token.
        let _if: syn::Token![if] = input.parse()?;
        // Parse the conditional expression.
        syn::Expr::parse(input).map(Self)
    }
}

impl syn::parse::Parse for Conditions {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(vec![]))
    }
}

impl syn::parse::Parse for ForIfClause {
    fn parse(input: ParseStream) -> Result<Self> {
        let _if: syn::Token![for] = input.parse()?;
        let pattern: Pattern = input.parse()?;
        let _in: syn::Token![in] = input.parse()?;
        let range: syn::Expr = input.parse()?;
        let condition: Conditions = input.parse()?;

        Ok(Self {
            pattern,
            range,
            condition,
        })
    }
}
