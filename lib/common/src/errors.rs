use anchor_lang::prelude::*;

/// Do NOT reorder the errors in this enum. Tests are relying on error ordering.
/// Not great, but for some reason when ErrorCode is factored out into a lib,
/// test messages no longer print actual messages and print error codes instead.
///
/// The other alternative is to have a custom error type inside the common library
/// and to try to convert that -> ErrorCode -> ProgramError
/// Unfortunately I wasn't able to get that working, last leg is failing.
///
/// todo to revisit in v1
#[error_code]
pub enum ErrorCode {
    // --------------------------------------- generic (0 - 19)
    #[msg("unknown instruction called")]
    UnknownInstruction,

    #[msg("anchor serialization issue")]
    AnchorSerializationIssue,

    #[msg("invalid permission passed")]
    InvalidPermission,

    #[msg("wrong permission type")]
    WrongPermission,

    #[msg("unauthorized master")]
    UnauthorizedMaster,

    #[msg("proof key mismatch")]
    ProofKeyMismatch,

    #[msg("batch key mismatch")]
    BatchKeyMismatch,

    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
    Reserved11,
    Reserved12,
    Reserved13,
    Reserved14,
    Reserved15,
    Reserved16,
    Reserved17,
    Reserved18,
    Reserved19,
    Reserved20,
    Reserved21,
    Reserved22,
    Reserved23,
    Reserved24,
    Reserved25,
    Reserved26,
    Reserved27,
    Reserved28,
    Reserved29,
    Reserved30,
    Reserved31,
    Reserved32,
    Reserved33,
    Reserved34,
    Reserved35,
    Reserved36,
    Reserved37,
    Reserved38,
    Reserved39,
    Reserved40,
    Reserved41,
    Reserved42,
    Reserved43,
    Reserved44,
    Reserved45,
    Reserved46,
    Reserved47,
    Reserved48,
    Reserved49
}
