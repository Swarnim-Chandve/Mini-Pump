use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {

    pub admin: Pubkey,
    pub fee_bps: u16,
    pub treasury: Pubkey,
    pub bump: u8,
}