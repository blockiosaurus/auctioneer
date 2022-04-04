extern crate proc_macro;

use anchor_lang::prelude::*;
//use proc_macro::*;
use proc_macro2::{
    Ident,
    Span,
};
use quote::{
    ToTokens,
    quote,
};
use syn::{
    parse_macro_input,
    Token,
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
};
use std::collections::HashSet;

struct Features {
    vars: HashSet<Ident>,
}

impl Parse for Features {
    fn parse(input: ParseStream) -> std::result::Result<Self, syn::Error> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Features {
            vars: vars.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn auctioneer_modules(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    msg!(&args.to_string());

    let features = parse_macro_input!(args as Features);
    let init = generate_init(features);

    let feature_impl: proc_macro2::TokenStream = parse_macro_input!(input as auctioneer_syn::Program)
        .to_token_stream()
        .into();

    quote! {
        #init
        #feature_impl
    }.into()
}

fn generate_init(features: Features) -> proc_macro2::TokenStream {
    let mut timed_auction_code = quote!{};

    if features.vars.contains(&Ident::new("timed_auction", Span::call_site())) {
        timed_auction_code = quote!{
            timed_auction_initialize();
        };
    }

    quote! {
        pub fn initialize_features() {
            #timed_auction_code
        }
    }
}