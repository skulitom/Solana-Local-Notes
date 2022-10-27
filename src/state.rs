use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NoteState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub creator: Pubkey,
    pub title: String,
    pub text: String,
    pub latitude: f32,
    pub longitude: f32,
}

impl Sealed for NoteState {}

impl IsInitialized for NoteState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl NoteState {
    pub const DISCRIMINATOR: &'static str = "note";

    pub fn get_account_size(title: String, text: String) -> usize {
        return 
            (4 + NoteState::DISCRIMINATOR.len()) + 
            1 +
            32 +
            (4 + title.len()) + 
            (4 + text.len()) + 
            32 + 
            32;
    }
}
