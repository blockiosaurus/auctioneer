use crate::Program;

use quote::quote;
//use solana_program::clock::UnixTimestamp;

pub fn generate(_program: &Program) -> proc_macro2::TokenStream {
    quote! {
        pub fn timed_auction_initialize(
            ctx: Context<Initialize>,
            start_time: UnixTimestamp,
            end_time: UnixTimestamp
        ) -> ProgramResult {
            let auction_timing_config = &mut ctx.accounts.config;
            auction_timing_config.start_time = start_time;
            auction_timing_config.end_time = end_time;
            auction_timing_config.bump = *ctx.bumps.get("config").unwrap();

            Ok(())
        }

        // #[derive(Accounts)]
        // pub struct TimedAuctionInitialize<'info> {
        //     #[account(init, payer = signer, space = 8 + 8 + 8, seeds = [b"timed_auction", signer.key().as_ref()], bump)]
        //     pub config: Account<'info, AuctionTimingConfig>,
        //     #[account(mut)]
        //     pub signer: Signer<'info>,
        //     pub system_program: Program<'info, System>,
        // }

        //#[account]
        //#[derive(Default)]
        pub struct TimedAuctionConfig {
            start_time: UnixTimestamp,
            end_time: UnixTimestamp,
            bump: u8,
        }
    }
}
