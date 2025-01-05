use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenAccount, Mint, Token2022}
};
use crate::Offer;

use super::transfer_tokens;



#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, 

    #[account(mut)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=maker,
        space = 8 + Offer::INIT_SPACE, // 8 is the size of the discriminator
        seeds = [b"offer", maker.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub offer: Account<'info, Offer>,
    #[account(
        init_if_needed, 
        payer=maker, 
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

pub fn send_offered_tokens_to_vault(context: &Context<MakeOffer>, token_a_offered_amount: u64) -> Result<()> {
    transfer_tokens(
        &context.accounts.maker_token_account_a,
        &context.accounts.vault,
        token_a_offered_amount,
        &context.accounts.token_mint_a,
        &context.accounts.maker,
        &context.accounts.token_program,
    )?;

    Ok(())
}



pub fn save_offer(context: Context<MakeOffer>, id: u64, token_b_wanted_amount: u64) -> Result<()> {
    context.accounts.offer.set_inner(Offer{
        id,
        maker: context.accounts.maker.key(),
        token_mint_a: context.accounts.token_mint_a.key(),
        token_mint_b: context.accounts.token_mint_b.key(),
        token_b_wanted_amount,
        bump: context.accounts.offer.bump,
    });
    Ok(())
}