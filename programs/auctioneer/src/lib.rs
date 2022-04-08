use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
// use anchor_lang::prelude::{
//     Context,
//     Result,
//     msg,
// };
//TODO: Fix this so includes automatically happen
use solana_program::clock::UnixTimestamp;

//#[macro_use]
extern crate auctioneer_modules;

use auctioneer_modules::auctioneer_modules;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[auctioneer_modules(timed_auction)]
#[program]
pub mod auctioneer {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        //auctioneer_modules::feature_init_args!(),
    ) -> ProgramResult {
        let start_time: UnixTimestamp = 0;
        let end_time: UnixTimestamp = 0;
        initialize_features(ctx, start_time, end_time)?;
        msg!("Test");
        Ok(())
    }
}
