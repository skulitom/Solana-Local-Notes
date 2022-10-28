use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use crate::utils::Coord;

pub enum NoteInstructions {
    AddNote {
        title: String,
        text: String,
        latitude: Coord,
        longitude: Coord,
    },
}

#[derive(BorshDeserialize)]
struct NotePayload {
    title: String,
    text: String,
    latitude: Coord,
    longitude: Coord,
}

impl NoteInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        
        Ok(match variant {
            0 => {
                let payload = NotePayload::try_from_slice(rest).unwrap();
                Self::AddNote {
                    title: payload.title,
                    text: payload.text,
                    latitude: payload.latitude,
                    longitude: payload.longitude,
                }
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
