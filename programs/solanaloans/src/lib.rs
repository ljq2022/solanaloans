use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Em8pj36RxaefZ2cm9ZfcnyXedyemqbDVgTbGGcQMae5A");

#[program]
pub mod solanaloans {
  use super::*;
  pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
    Ok(())
  }

  // This function creates a loan.
  pub fn create_loan(ctx: Context<CreateLoan>) -> ProgramResult {
    let minimum_sol_balance = 2;
    let lamports_per_sol = 1000000000;
    let loan_amount = 2 * lamports_per_sol;
    let base_account = &mut ctx.accounts.base_account;
    let lamports = &base_account.to_account_info().lamports();

    // Return an error if the account does not have a sufficient balance to make a loan.
    if *lamports < minimum_sol_balance * lamports_per_sol {
      return Err(ProgramError::InsufficientFunds);
    }
    let user = &mut ctx.accounts.user;

    let loan_struct = LoanStruct {
        amount: 5,
        is_paid: false
    };

    // Iterate through the users that have already taken loans to see if the current user has already taken a loan before.
    let user_exists = &mut false;
    let existing_user_idx = &mut 0;
    for (i, iterated_user) in base_account.users.iter_mut().enumerate() {
        if *user.to_account_info().key.to_string() == iterated_user.key.to_string() {
          *user_exists = true;
          *existing_user_idx = i;
          break;
        }
    }
    // If the current user already has taken a loan, modify their existing loans array.
    // Otherwise, create a new user structure with a new loans array and add the user structure to the account users.
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
      let default_loan_struct = LoanStruct {
        amount: 0,
        is_paid: true
      };
      if user_struct.loans.last().unwrap_or(&default_loan_struct).is_paid == false {
        return Err(ProgramError::AccountBorrowFailed);
      }
      user_struct.loans.push(loan_struct);
      msg!("Enter user exists.");
    }
    // Make the transaction.
    **base_account.to_account_info().try_borrow_mut_lamports()? -= loan_amount;
    **user.to_account_info().try_borrow_mut_lamports()? += loan_amount;
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

// Add the signer who calls the CreateLoan method
#[derive(Accounts)]
pub struct CreateLoan<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// User data model.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserStruct {
  pub key: Pubkey,
  pub loans: Vec<LoanStruct>
}

#[account]
pub struct BaseAccount {
  pub users: Vec<UserStruct>
}

// Loan data model.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct LoanStruct {
  pub amount: u64,
  pub is_paid: bool
}
