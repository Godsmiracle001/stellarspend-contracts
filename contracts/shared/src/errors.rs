use soroban_sdk::contracterror;
/// Common cross-contract errors.
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SharedError {
    Unauthorized = 1,
    InvalidAmount = 2,
    InvalidAddress = 3,
    InvalidString = 4,
    Overflow = 5,
}
