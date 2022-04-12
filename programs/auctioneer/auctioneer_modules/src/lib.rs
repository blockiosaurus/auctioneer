extern crate proc_macro;

//use anchor_lang::prelude::*;
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
//use auctioneer_syn::codegen::program;

#[derive(Clone)]
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
    let features = parse_macro_input!(args as Features);
    let account_macro = generate_init_accounts(&features);
    //let args_macro = generate_init_args(&features);
    let init = generate_init(&features);

    let feature_impl: proc_macro2::TokenStream = parse_macro_input!(input as auctioneer_syn::Program)
        .to_token_stream()
        .into();

    let result = quote!{
        #account_macro
        //#args_macro

        #init
        
        #feature_impl
    };
    result.into()
}

fn generate_init(features: &Features) -> proc_macro2::TokenStream {
    let mut timed_auction_code = quote!{};

    if features.vars.contains(&Ident::new("timed_auction", Span::call_site())) {
        timed_auction_code = quote! {
            timed_auction_initialize(ctx, start_time, end_time)?;
        };
    }

    quote! {
        macro_rules! init_features {
            () => {
                pub fn initialize_features(
                    ctx: Context<Initialize>,
                    start_time: UnixTimestamp,
                    end_time: UnixTimestamp,
                ) -> ProgramResult {
                    #timed_auction_code

                    Ok(())
                }               
            };
        }
    }
}

fn generate_init_accounts(features: &Features) -> proc_macro2::TokenStream {
    let mut timed_auction_code = quote!{};
    let mut min_price = quote!{};

    if features.vars.contains(&Ident::new("timed_auction", Span::call_site())) {
        timed_auction_code = quote! {
            #[account(init, payer = signer, space = 8 + 8 + 8 + 1, seeds = [b"timed_auction", signer.key().as_ref()], bump)]
            pub config: Account<'info, TimedAuctionConfig>,
            #[account(mut)]
            pub signer: Signer<'info>,
            pub system_program: Program<'info, System>,
        };
    }

    if features.vars.contains(&Ident::new("min_price", Span::call_site())) {
        timed_auction_code = quote! {
            #[account(init, payer = signer, space = 8 + 8 + 8 + 1, seeds = [b"timed_auction", signer.key().as_ref()], bump)]
            pub config: Account<'info, TimedAuctionConfig>,
            #[account(mut)]
            pub signer: Signer<'info>,
            pub system_program: Program<'info, System>,
        };
    }

    quote! {
        //macro_rules! feature_init_accounts {
        //    () => {
                #[derive(Accounts)]
                pub struct Initialize<'info> {
                    #timed_auction_code
                    #min_price
                }
        //    };
        //}
    }
}

// fn generate_init_args(features: &Features) -> proc_macro2::TokenStream {
//     let mut timed_auction_code = quote!{};

//     if features.vars.contains(&Ident::new("timed_auction", Span::call_site())) {
//         timed_auction_code = quote! {
//             timed_auction_init: program::TimedAuctionConfig,
//         };
//     }

//     quote! {
//         macro_rules! feature_init_args {
//             () => {
//                 #timed_auction_code,
//             };
//         }
//     }
// }