use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicsolanaproject {
  use super::*;
  pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> Result <()> {
    // Get a reference to the account.
    let base_account = &mut _ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  // Another function woo!
  pub fn add_gif(_ctx: Context<AddGif>) -> Result <()> {
    // Get a reference to the account and increment total_gifs.

    let base_account = &mut _ctx.accounts.base_account;

    base_account.total_gifs += 1;
    Ok(())
  }

}

// Attach certain variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init ,payer = user, space = 9000)]
    pub base_account: Account<'info,BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info,System>,
}

// Specify what data you want in the AddGif Context
#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
