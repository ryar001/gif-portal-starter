
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// basic anchor definition
// #[program] is where the function for the logic of the program is written
// #[derive(Accounts)] is where is put our structs
// #[account] 

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  // input ctx of type Context<StartStuffOff>  and output ProgramResult
  // ctx is main way for endpoints to connect to the accounts Struct
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult{
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    // Build the ItemStruct
    let item = ItemStruct {
      gif_link: gif_link.to_string(), // set the link to string type
      user_address: *user.to_account_info().key,
    };

    // add item to the gif_list vector
    base_account.gif_list.push(item);
    base_account.total_gifs+=1;
    Ok(())
  }
}




/*First we've got [account(init, payer = user, space = 9000)]. 
All we're doing here is telling Solana how we want to initialize BaseAccount.
Note, if after running your test below you get the error Transaction simulation failed: Error processing Instruction 0: custom program error: 0x64, you will need to change space = 9000 to space = 10000. If you look at these docs from anchor you can see that they define a simple program that declares space = 8 + 8 (eg, 8 kilobytes + 8 kilobytes). The more logic we add to our program, the more space it will take up!
  - init will tell Solana to create a new account owned by our current program.
  - payer = user tells our program who's paying for the account to be created. In this case, it's the user calling the function.
  - We then say space = 9000 which will allocate 9000 bytes of space for our account. You can change this # if you wanted, but, 9000 bytes is enough for the program we'll be building here!
*/
// The Account type is used when an instruction 
// is interested in the deserialized data of the account. 
// Consider the following example where we set some data in an account: 
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // 
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    
    // get user signature to verify wallet
    #[account(mut)]
    pub user: Signer<'info>,
    // system Program is the program controlling the solana cluster
    // one of the function it does is creating acct as down below
    pub system_program: Program <'info, System>,
}
// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account : Account<'info,BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}


// Serialize and deserialise to binary format to store into account
#[derive(Debug,Clone, AnchorSerialize,AnchorDeserialize)]
pub struct ItemStruct {
  pub gif_link: String,
  pub user_address: Pubkey
}


// tell solana the type the of data the account is going to store
// below we instruct our program to make an acct of 64 byte of unsigned int 
// to store out gif in total_gif
#[account]
pub struct BaseAccount {
  pub total_gifs: u64,
  // add another variable to the account with Vec type
  // storing data of ItemStruct which we will define above
  // essentially a list or array
  pub gif_list: Vec<ItemStruct>,
  pub dice_result: u64,
}