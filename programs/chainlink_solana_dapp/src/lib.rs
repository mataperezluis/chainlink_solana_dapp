use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use chainlink_solana as chainlink;
use anchor_lang::solana_program::system_program;

declare_id!("4mV9cwBikDuxu6KLkm7mpTasidbDD3uebevzB5DGPyws");

#[program]
pub mod chainlink_solana_dapp {
    use super::*;

    pub fn execute(ctx: Context<Execute>) -> ProgramResult {
        let round = chainlink::latest_round_data(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed.to_account_info())?;
            let result_account = &mut ctx.accounts.result_account;
            result_account.value = round.answer;
            Ok(())
        
    }
}

#[account]
pub struct ResultAccount{
    pub value:i128
}

#[derive(Accounts)]
pub struct Execute<'info>
{   #[account(init,payer=user,space=100)]
    pub result_account: Account<'info, ResultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK:
    pub system_program: AccountInfo<'info>,
    /// CHECK:
    pub chainlink_program: AccountInfo<'info>,
    /// CHECK:
    pub chainlink_feed: AccountInfo<'info>
}


