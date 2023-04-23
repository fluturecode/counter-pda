use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Counter {
    pub value: u8,
}

impl Sealed for Counter {}

impl Pack for Counter {
    const LEN: usize = 1;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.value;
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        Ok(Self { value: src[0] })
    }
}

impl IsInitialized for Counter {
    fn is_initialized(&self) -> bool {
        self.value != 0
    }
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program invoked");

    // Parse accounts
    let accounts_iter = &mut accounts.iter();
    let counter_account = next_account_info(accounts_iter)?;

    // Dispatch based on instruction
    match instruction_data[0] {
        0 => {
            msg!("Creating counter account");
            if counter_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            if !counter_account.data_is_empty() {
                return Err(ProgramError::AccountAlreadyInitialized);
            }
            let counter_data = Counter { value: 0 };
            counter_data.pack_into_slice(&mut counter_account.data.borrow_mut());
            Ok(())
        }
        1 => {
            msg!("Incrementing counter account");
            if counter_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let mut counter_data = Counter::unpack(&counter_account.data.borrow())?;
            counter_data.value = counter_data.value.wrapping_add(1);
            counter_data.pack_into_slice(&mut counter_account.data.borrow_mut());
            Ok(())
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
