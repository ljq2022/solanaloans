use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Em8pj36RxaefZ2cm9ZfcnyXedyemqbDVgTbGGcQMae5A");

#[program]
pub mod solanaloans {
  use super::*;
  pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
    Ok(())
  }
  
  pub fn pay_loan(ctx: Context<PayLoan>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    // Iterate through the users that have already taken loans to see if the current user has taken a loan.
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
      return Err(ProgramError::UninitializedAccount);
    }
    let user_struct = &mut base_account.users[*existing_user_idx];
    // We initialize a default loan structure because the `last()` function in a vec returns an Optional type
    // because the result of `last()` could be None if the array is empty.
    // For type safety, `unwrap_or` ensures that there will always be a loan object in the conditional.
    let default_loan_struct = LoanStruct {
      amount: 0,
      is_paid: true,
      repayment_amount: 0,
      creation_time: Clock::get().unwrap().unix_timestamp
    };
    if user_struct.loans.last().unwrap_or(&default_loan_struct).is_paid {
      return Err(ProgramError::InvalidAccountData);
    }
    let most_recent_loan = user_struct.loans.pop().unwrap_or(default_loan_struct);
    let updated_loan = LoanStruct {
      amount: most_recent_loan.amount,
      is_paid: true,
      repayment_amount: most_recent_loan.repayment_amount,
      creation_time: most_recent_loan.creation_time
    };
    user_struct.loans.push(updated_loan);
    // Make the transaction.
    let ix = anchor_lang::solana_program::system_instruction::transfer(
      &user.key(),
      &base_account.key(),
      most_recent_loan.repayment_amount,
    );
    return anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            user.to_account_info(),
            base_account.to_account_info(),
        ],
    );
  }

  // This function creates a loan.
  pub fn create_loan(ctx: Context<CreateLoan>) -> ProgramResult {
    let minimum_sol_balance = 2;
    let lamports_per_sol: u64 = 1000000000;
    let loan_amount: u64 = 2 * lamports_per_sol;
    let loan_repayment_amount: u64 = 3 * lamports_per_sol;
    let base_account = &mut ctx.accounts.base_account;
    let lamports = &base_account.to_account_info().lamports();
    let current_time = Clock::get().unwrap().unix_timestamp;

    // Return an error if the account does not have a sufficient balance to make a loan.
    if *lamports < minimum_sol_balance * lamports_per_sol {
      return Err(ProgramError::InsufficientFunds);
    }
    let user = &mut ctx.accounts.user;

    let loan_struct = LoanStruct {
        amount: loan_amount,
        is_paid: false,
        repayment_amount: loan_repayment_amount,
        creation_time: current_time
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
      // The next conditional checks if a user has paid back their most recent loan. If they haven't, return an error.
      // For why we use the `unwrap_or` pattern, see the explanation above in `pay_loan()`
      let default_loan_struct = LoanStruct {
        amount: 0,
        is_paid: true,
        repayment_amount: 0,
        creation_time: current_time
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

#[derive(Accounts)]
pub struct CreateLoan<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct PayLoan<'info> {
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
  pub is_paid: bool,
  pub repayment_amount: u64,
  pub creation_time: i64
}
