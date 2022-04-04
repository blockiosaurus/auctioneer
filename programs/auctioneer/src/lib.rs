use anchor_lang::prelude::*;
//TODO: Fix this so includes automatically happen
use solana_program::clock::UnixTimestamp;

#[macro_use]
extern crate auctioneer_modules;

use auctioneer_modules::auctioneer_modules;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[auctioneer_modules(timed_auction)]
#[program]
pub mod auctioneer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Test");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
