use codegen::program as program_codegen;
use parser::program as program_parser;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Error as ParseError, Parse, ParseStream, Result as ParseResult};
use syn::{
    Expr, Generics, Ident, ImplItemMethod, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, LitInt,
    LitStr, PatType, Token, TypePath,
};

pub mod codegen;
pub mod parser;

#[derive(Debug)]
pub struct Program {
    //pub state: Option<State>,
    //pub ixs: Vec<Ix>,
    pub name: Ident,
    pub program_mod: ItemMod,
    //pub fallback_fn: Option<FallbackFn>,
}

impl Parse for Program {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let program_mod = <ItemMod as Parse>::parse(input)?;
        program_parser::parse(program_mod)
    }
}

impl From<&Program> for TokenStream {
    fn from(program: &Program) -> Self {
        program_codegen::generate(program)
    }
}

impl ToTokens for Program {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}
