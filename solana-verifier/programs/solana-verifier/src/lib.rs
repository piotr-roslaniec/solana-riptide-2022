mod verifier;

use anchor_lang::prelude::*;

declare_id!("ETYYD1fm5fNoTe9HnHYbc9yf9oFZ2kbqEmBvbRwnAPzv");

#[program]
pub mod solana_verifier {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vk = verifier::make_verifying_key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
