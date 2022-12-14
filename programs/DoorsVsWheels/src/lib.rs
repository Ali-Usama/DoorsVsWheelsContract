use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke};
use anchor_lang::solana_program::system_instruction::{transfer};


declare_id!("B9BEfW5pyg4FGJjkRTc3t8rgSzxP3WPp2cf8BPx57Xjn");

#[program]
pub mod doors_vs_wheels {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let votes_counter = &mut ctx.accounts.votes_counter;
        votes_counter.doors = 0;
        votes_counter.wheels = 0;
        msg!("Votes Counter Initialized");
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote: VoteOptions) -> Result<()> {
        let votes_counter = &mut ctx.accounts.votes_counter;

        match vote {
            VoteOptions::Doors => votes_counter.doors += 1,
            VoteOptions::Wheels => votes_counter.wheels +=1,
        }

        let user_vote = &mut ctx.accounts.user_vote;
        user_vote.bump = *ctx.bumps.get("user_vote").unwrap();
        user_vote.vote = vote;

        let voting_fee:u64 = 1_000_000_000;     // Fee in lamports

        let voting_fee_transfer = transfer(
            &ctx.accounts.user.key(),
            &votes_counter.key(),
            voting_fee
        );

        invoke(
            &voting_fee_transfer,
            &[
                ctx.accounts.user.to_account_info(),
                votes_counter.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(init, payer=user, space=8+ 2*8, seeds=[b"votes_counter".as_ref()], bump)]
    votes_counter: Account<'info, VotesCounter>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(vote: VoteOptions)]   // defining that it expects a parameter name 'vote' of type 'VoteOptions'
pub struct Vote<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer=user, space=8+1+2,
    seeds=[b"user_vote".as_ref(), user.key().as_ref()], bump)]
    pub user_vote: Account<'info, UserVote>,
    #[account(mut)]
    pub votes_counter: Account<'info, VotesCounter>,
    pub system_program: Program<'info, System>
}


// Data Storage account
#[account]
pub struct VotesCounter {
    pub doors: u64,
    pub wheels: u64,
}

#[account]
pub struct UserVote {
    bump: u8,
    vote: VoteOptions
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteOptions {
    Doors,
    Wheels,
}