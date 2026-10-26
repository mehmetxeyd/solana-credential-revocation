use anchor_lang::prelude::*;

declare_id!("HNGzq89ne642qkLDCszQ56Y6PKGgBSRTpmPHG3JRYcdX");

#[program]
pub mod credential_revocation {
    use super::*;

    // Records that the issuer has revoked the credential with this hash.
    // The record's address is derived from the issuer and the hash, so
    // anyone can look it up, and only the issuer can create it.
    pub fn revoke(ctx: Context<Revoke>, credential_hash: [u8; 32]) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.issuer = ctx.accounts.issuer.key();
        record.credential_hash = credential_hash;
        record.revoked_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(credential_hash: [u8; 32])]
pub struct Revoke<'info> {
    #[account(
        init,
        payer = issuer,
        space = 8 + 32 + 32 + 8,
        seeds = [b"revoked", issuer.key().as_ref(), credential_hash.as_ref()],
        bump
    )]
    pub record: Account<'info, RevocationRecord>,
    #[account(mut)]
    pub issuer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RevocationRecord {
    pub issuer: Pubkey,
    pub credential_hash: [u8; 32],
    pub revoked_at: i64,
}
