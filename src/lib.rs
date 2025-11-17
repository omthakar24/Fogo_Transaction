use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    system_instruction,
    program::invoke,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("FOGO PROGRAM: transferring 0.1 FOGO");

    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let destination = next_account_info(accounts_iter)?;

    if !payer.is_signer {
        msg!("ERROR: payer must sign");
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    let lamports: u64 = 100_000_000;

    msg!("Transferring {} lamports", lamports);

    invoke(
        &system_instruction::transfer(
            payer.key,
            destination.key,
            lamports,
        ),
        &[payer.clone(), destination.clone()],
    )?;

    msg!("Transfer DONE");

    Ok(())
}
