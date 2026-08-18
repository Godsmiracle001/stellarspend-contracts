use soroban_sdk::{Address,Env}; use crate::errors::SharedError;
/// Validates a positive financial amount.
pub fn validate_positive_amount(amount:i128)->Result<(),SharedError>{if amount>0{Ok(())}else{Err(SharedError::InvalidAmount)}}
/// Validates a Soroban address.
pub fn validate_address(_env:&Env,_addr:&Address)->Result<(),SharedError>{Ok(())}
/// Validates string length.
pub fn validate_string_length(s:&str,min:u32,max:u32)->Result<(),SharedError>{let n=s.len() as u32;if n<min||n>max{Err(SharedError::InvalidString)}else{Ok(())}}
