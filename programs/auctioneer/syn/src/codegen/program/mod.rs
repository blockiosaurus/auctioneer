use crate::Program;
use quote::quote;
//use anchor_lang;

//mod accounts;
//pub mod common;
//mod cpi;
//mod dispatch;
//mod entry;
//mod handlers;
//mod instruction;
mod timed_auction;

pub fn generate(program: &Program) -> proc_macro2::TokenStream {
    let mod_name = &program.name;

    let timed_auction = timed_auction::generate(program);

    //let entry = entry::generate(program);
    //let dispatch = dispatch::generate(program);
    //let handlers = handlers::generate(program);
    let user_defined_program = &program.program_mod;
    //let instruction = instruction::generate(program);
    //let cpi = cpi::generate(program);
    //let accounts = accounts::generate(program);

    quote! {
        // TODO: remove once we allow segmented paths in `Accounts` structs.
        use self::#mod_name::*;

        #timed_auction

        //#entry
        //#dispatch
        //#handlers
        #user_defined_program
        //#instruction
        //#cpi
        //#accounts
    }
}