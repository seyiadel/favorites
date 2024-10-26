use anchor_lang::prelude::*;

declare_id!("aJnkVaqCf69jue4bwWUY6JR23WWDirovXJqdYxd41o2");

pub const ANCHOR_DISCRIMINATOR_SIZE:usize = 8;
#[program]
pub mod favorites {

    use super::*;

    pub fn set_favorites(ctx:Context<SetFavorites>, number:u64, color:String, hobbies:Vec<String> )-> Result<()> {
        msg!("Hi, This is the favorite program {}", ctx.program_id);
        let user_public_key = ctx.accounts.user.key();
        msg!("User Public Key : {}", user_public_key);
        msg!("{color}, {number}, {hobbies:?}" );


        ctx.accounts.favorites.set_inner(Favorites { 
            number,
            color, 
            hobbies, 
        });

        Ok(())
    }
}

// Favorite things to be saved
#[account]
#[derive(InitSpace)] //Size of the object
pub struct Favorites {
    pub number:u64,

    #[max_len(50)]
    pub color:String,

    #[max_len(5, 50)]
    pub hobbies:Vec<String>,
}


// SetFavourites Account 
#[derive(Accounts)]
pub struct SetFavorites<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        payer=user,
        space=ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds=[b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites:Account<'info, Favorites>,

    pub system_program:Program<'info, System>,

}
