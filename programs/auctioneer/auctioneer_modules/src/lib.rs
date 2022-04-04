extern crate proc_macro;

use anchor_lang::prelude::*;
use proc_macro::*;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn auctioneer_modules(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    msg!(&_args.to_string());
    parse_macro_input!(input as auctioneer_syn::Program)
        .to_token_stream()
        .into()
}
