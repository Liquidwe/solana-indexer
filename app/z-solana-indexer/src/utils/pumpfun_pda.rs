use std::str::FromStr;

use solana_program::{
    pubkey::{Pubkey, PubkeyError},
    pubkey,
};

pub const PUMP_FUN_PROGRAM: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

pub const PUMP_FUN_ACCOUNT: Pubkey = pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");

pub const ASSOCIATED_TOKEN_PROGRAM: Pubkey = pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

pub fn get_bonding_curve_by_mint(mint: &String) -> Result<(Pubkey, Pubkey), PubkeyError> {
    let mint_address = Pubkey::from_str(mint).unwrap();

    let (bonding_curve, _) = Pubkey::find_program_address(
        &[b"bonding-curve", mint_address.as_ref()],
        &PUMP_FUN_PROGRAM,
    );

    let (associated_bonding_curve, _) = Pubkey::find_program_address(
        &[bonding_curve.as_ref(), PUMP_FUN_ACCOUNT.as_ref(), mint_address.as_ref()],
        &ASSOCIATED_TOKEN_PROGRAM,
    );

    Ok((bonding_curve, associated_bonding_curve))
} 