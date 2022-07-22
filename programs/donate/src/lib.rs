use anchor_lang::prelude::*;

declare_id!("7hTffwGm6YkPK5rC4N1LfSQvZkr97ZvGQX4qVY7w77ht");

#[program]
pub mod donate {
    use super::*;

    pub fn setup(ctx: Context<Setup>) -> Result<()> {
        let summary = &mut ctx.accounts.summary;        
        summary.donations = 0_u64;
        summary.total = 0_u64;
        summary.bump = *ctx.bumps.get("summary").unwrap();
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u32) -> Result<()> {
        let donation = &mut ctx.accounts.donation;
        let summary = &mut ctx.accounts.summary;

        donation.bump = *ctx.bumps.get("donation").unwrap();
        donation.amount = amount;
        summary.add(DonorAmount { amount: amount, donor: ctx.accounts.donor.key() });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8 + 8 + ((4+32) * 10), seeds = [b"donation-summary", id().as_ref()], bump)]
    pub summary: Account<'info, DonationSummary>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(init, payer=donor, space= 8 + 1 + 4, seeds=[b"donation", donor.key.as_ref()], bump)]
    pub donation: Account<'info, Donation>,
    
    #[account(mut)]
    pub donor: Signer<'info>,

    #[account(mut, seeds = [b"donation-summary", id().as_ref()], bump=summary.bump)]
    pub summary: Account<'info, DonationSummary>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct Donation{
    pub bump: u8, //1 byte
    pub amount: u32, //4 bytes
}

#[account]
pub struct DonationSummary{
    pub bump: u8, //1 byte
    pub donations: u64, //8 bytes
    pub total: u64, //8 bytes
    pub top10: [DonorAmount; 10] // (4+32) * 10    
}

impl DonationSummary {
    pub fn add(&mut self, donation: DonorAmount) {        
        self.total += donation.amount as u64;
        self.donations += 1;
        self.update_top10(donation);    
    }

    pub fn update_top10(&mut self, donor_amount: DonorAmount){
        let mut insert_index: Option<usize> = None;

        for i in 0..self.top10.len() {
            if donor_amount.amount > self.top10[i].amount {
                insert_index = Some(i);
                break;
            }
        }

        if let Some(index) = insert_index {
            let start = index+1;
            for i in (start..self.top10.len()).rev() {
                self.top10[i] = self.top10[i-1];
            }

            self.top10[index] = donor_amount;
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Copy)]
pub struct DonorAmount{
    pub amount: u32, //4
    pub donor: Pubkey, //32
}
