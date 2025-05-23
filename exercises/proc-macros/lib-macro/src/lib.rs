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

// In the beginning we need proc_macro2 and when we are finished, we
// turn these crate to `proc_macro` which is the compiler internal crate.
type TokenStream2 = proc_macro2::TokenStream;

use quote::quote;
use syn::{parse::ParseStream, parse_macro_input};

/// This is a comprehension written as e.g.
/// `[ x*x for x in x[0..2] ]`
///        --------------- := `for_if_clause`
///    --- : `mapping`
///
/// This struct implements `parse` as all other
/// downstream ones.
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
    if_clauses: IfClauses,
}

#[derive(Debug)]
struct Pattern(syn::Pat);

#[derive(Debug)]
struct IfClauses(Vec<IfClause>);

#[derive(Debug)]
struct IfClause(syn::Expr);

impl syn::parse::Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}

impl syn::parse::Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Expr::parse(input).map(Self)
    }
}

impl syn::parse::Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _for: syn::Token![for] = input.parse()?;
        let pattern: Pattern = input.parse()?;
        let _in: syn::Token![in] = input.parse()?;
        let range: syn::Expr = input.parse()?;
        let if_clauses: IfClauses = input.parse()?;

        Ok(Self {
            pattern,
            range,
            if_clauses,
        })
    }
}

impl syn::parse::Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
    }
}

impl syn::parse::Parse for IfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the 'if' token.
        let _if: syn::Token![if] = input.parse()?;
        // Parse the conditional expression.
        syn::Expr::parse(input).map(Self)
    }
}

impl syn::parse::Parse for IfClauses {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(parse_zero_or_more::<IfClause>(input)?))
    }
}

fn parse_zero_or_more<T: syn::parse::Parse>(
    input: ParseStream,
) -> syn::Result<Vec<T>> {
    let mut res = Vec::new();

    while !input.is_empty() {
        match input.parse::<T>() {
            Ok(item) => res.push(item),
            Err(e) => {
                syn::Error::new(
                    e.span(),
                    "could not parse `if ...` statement in `comp!`!",
                );
                return Err(e);
            }
        }
    }

    Ok(res)
}

/// Implementing quoting for [] which turns this into expression
/// into Rust code again.
impl quote::ToTokens for Comprehension {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // Turning the parsed structures from `comprehnsion![ x*2 for x in [1, 2, 3, 4] ]` into
        // a token stream of equivalent Rust code.
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            range,
            if_clauses,
        } = &self.for_if_clause;

        // Create an iterator over various `if-expressions` (TokenStream2).
        let ifs = if_clauses.0.iter().map(|c| {
            quote! { #c }
        });

        let s = quote! {
            core::iter::IntoIterator::into_iter(#range).filter_map(|#pattern| {
                (true #(&& (#ifs))*).then(|| #mapping)
            })
        };

        tokens.extend(s);
    }
}

/// Turn [`Pattern`] expression into Rust code again.
impl quote::ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

/// Turn [`IfClause`] expression into Rust code again.
impl quote::ToTokens for IfClause {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Comprehension);
    let s = quote! {#c}.into();
    eprintln!(
        "=======================\n\
        Proc-macro generated code:\n\
        '{}'\n\
        ========================",
        s
    );

    s
}
