use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

solana_program::declare_id!("YckqussvUgFrrmDru4mdduJNaeUDSoUNhxmp6Wk44EY");

solana_program::entrypoint!(process_instruction);

// Adjust this constant to test behavior with different numbers of keys.
pub const NUM_KEYS: usize = 14;

struct MockSignerStruct<'a>(pub AccountInfo<'a>);

impl<'a> MockSignerStruct<'a> {
    fn try_from(account: &'a AccountInfo<'a>) -> Result<Self, ProgramError> {
        if account.is_signer {
            Ok(MockSignerStruct(account.clone()))
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }

    fn try_accounts(accounts: &mut &'a [AccountInfo<'a>]) -> Result<Self, ProgramError> {
        if accounts.is_empty() {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let account = &accounts[0];
        *accounts = &accounts[1..];
        MockSignerStruct::try_from(account)
    }
}

fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    _input: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let rent = <Rent as Sysvar>::get()?;

    let payer = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;

    let mut remaining_accounts: &mut &'a [AccountInfo<'a>] = &mut &accounts[2..];

    for _ in 0..NUM_KEYS {
        msg!("Remaining accounts: {:?}", remaining_accounts.len());
        let signer = MockSignerStruct::try_accounts(&mut remaining_accounts)?;
        invoke(
            &solana_program::system_instruction::create_account(
                &payer.key,
                &signer.0.key,
                rent.minimum_balance(8),
                8,
                program_id,
            ),
            &[signer.0.clone(), payer.clone()],
        )?;
    }

    Ok(())
}
