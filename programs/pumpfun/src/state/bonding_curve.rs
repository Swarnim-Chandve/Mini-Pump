use anchor_lang::prelude::*;

#[account]
pub struct Market {
    pub mint: Pubkey,
    pub supply: u64,
    pub reserve_lamports: u64,
    pub bump: u8,
}


impl Market {
    pub fn buy_price_for(&self, quantity: u64) -> u64 {

    }


    pub fn sell_revenue_for(&self, quantity: u64) -> u64 {
        
    }
}