extern crate proc_macro;

use combine::parser::Parser;
use combine_proc_macro::{Input, Incomplete};
use crabwise_antlr_parser::grammar;
use proc_macro::TokenStream;
use proc_macro2::{Ident as Ident2, Span as Span2};
use quote::quote;

#[proc_macro]
pub fn const_antlr_ast_text(input: TokenStream) -> TokenStream {
    let input = Input::from(input).with_lookahead(4);
    let result = grammar::grammar().easy_parse(input);
    let (ast, trailing) = match result {
        Ok(ok) => ok,
        Err(err) => panic!("error parsing grammar definition: {:#?}", err),
    };
    if let Some(diagnostic) = Incomplete::from_stream(trailing) {
        panic!("unexpected tokens at end of grammar definition:\n\n{}", diagnostic);
    }

    let name = Ident2::new(&ast.name.to_string(), Span2::from(ast.name.span()));
    let ast_str = format!("{:#?}", ast);
    let result = quote! { const #name: &'static str = #ast_str; };
    result.into()
}
