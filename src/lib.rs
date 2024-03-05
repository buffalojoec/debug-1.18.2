use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

solana_program::declare_id!("YckqussvUgFrrmDru4mdduJNaeUDSoUNhxmp6Wk44EY");

solana_program::entrypoint!(process_instruction);

// Adjust this constant to test behavior with different numbers of keys.
pub const NUM_KEYS: usize = 14;

fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _input: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let rent = <Rent as Sysvar>::get()?;

    let signer = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;

    for _ in 0..NUM_KEYS {
        let account = next_account_info(accounts_iter)?;
        invoke(
            &solana_program::system_instruction::create_account(
                &signer.key,
                &account.key,
                rent.minimum_balance(8),
                8,
                program_id,
            ),
            &[account.clone(), signer.clone()],
        )?;
    }

    Ok(())
}
