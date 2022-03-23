use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Em8pj36RxaefZ2cm9ZfcnyXedyemqbDVgTbGGcQMae5A");

#[program]
pub mod solanaloans {
  use super::*;
  pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_sol = 0;
    Ok(())
  }

  // This function creates a loan.
  pub fn create_loan(ctx: Context<CreateLoan>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    // if base_account.total_sol < 5 {
    //   return Err(ProgramError::InsufficientFunds);
    // }
    msg!(base_account);
    let user = &mut ctx.accounts.user;

    let loan_struct = LoanStruct {
        amount: 5,
        is_paid: false
    };

    let user_exists = &mut false;
    let existing_user_idx = &mut 0;
    for (i, iterated_user) in base_account.users.iter_mut().enumerate() {
        if *user.to_account_info().key.to_string() == iterated_user.key.to_string() {
          *user_exists = true;
          *existing_user_idx = i;
          break;
        }
    }
    if *user_exists == false {
      let mut loans = Vec::new();
      loans.push(loan_struct);
      
      let user_struct = UserStruct {
        key: *user.to_account_info().key,
        loans: loans
      };
      base_account.users.push(user_struct);
      msg!("Enter user does not exist.");
    } else {
      let user_struct = &mut base_account.users[*existing_user_idx];
      user_struct.loans.push(loan_struct);
      msg!("Enter user exists.");
    }
    base_account.total_sol -= 5;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct CreateLoan<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// User data model.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserStruct {
  pub key: Pubkey,
  pub loans: Vec<LoanStruct>
}

#[account]
pub struct BaseAccount {
  pub total_sol: u64,
  pub users: Vec<UserStruct>
}

// Loan data model.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct LoanStruct {
  pub amount: u64,
  pub is_paid: bool
}
