pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3XomgAn7joYbdhVsYCo7RcBA4WgF3vF8KCS6AfFJXNL3");

#[program]
pub mod swap {
    use anchor_spl::token;

    use super::*;

    pub fn make_offer(context: Context<MakeOffer>, id:u64, token_a_offered_amount:u64, token_b_wanted_amount:u64) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&context, token_a_offered_amount)?;
        instructions::make_offer::save_offer(context, id, token_b_wanted_amount)
        // Ok(())
    }
}
