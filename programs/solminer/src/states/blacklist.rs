use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Blacklist {
    pub addresses: Vec<Pubkey>,
}
