use crate::error::ReviewError;
use crate::instruction::NoteInstructions;
use crate::state::{NoteState};
use crate::utils::Coord;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NoteInstructions::unpack(instruction_data)?;
    match instruction {
        NoteInstructions::AddNote {
            title,
            text,
            latitude,
            longitude,
        } => add_new_note(program_id, accounts, title, text, latitude, longitude),
    }
}

pub fn add_new_note(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    text: String,
    latitude: Coord,
    longitude: Coord,
) -> ProgramResult {
    msg!("Creating new note with latitude {} and longitude {}", latitude, longitude);
    msg!("Title {} and Text {}", title, text);

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let account_len: usize = 1000;
    if NoteState::get_account_size(title.clone(), text.clone()) > account_len {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<NoteState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    msg!("checking if movie account is already initialized");
    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.discriminator = NoteState::DISCRIMINATOR.to_string();
    account_data.creator = *initializer.key;
    account_data.title = title;
    account_data.text = text;
    account_data.latitude = latitude;
    account_data.longitude = longitude;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}
