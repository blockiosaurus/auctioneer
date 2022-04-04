use crate::Program;
use syn::parse::{Error as ParseError, Result as ParseResult};

pub fn parse(program_mod: syn::ItemMod) -> ParseResult<Program> {
    //let state = state::parse(&program_mod)?;
    //let (ixs, fallback_fn) = instructions::parse(&program_mod)?;
    Ok(Program {
        // state,
        // ixs,
        name: program_mod.ident.clone(),
        program_mod,
        // fallback_fn,
    })
}