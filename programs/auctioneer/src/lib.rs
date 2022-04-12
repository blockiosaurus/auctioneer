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

//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[auctioneer_modules(timed_auction)]
//#[program]
pub mod auctioneer {
    use super::*;

    init_features!();

    fn buy() {
        timed_auction_assert_active()
    }
}
