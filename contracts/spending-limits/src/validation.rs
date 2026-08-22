use soroban_sdk::Symbol;

use crate::Error;

/// Validates a spending-limit or spend amount: must be strictly positive.
/// Reuses `shared::validation::validate_positive_amount`, mapped onto this
/// contract's own typed error.
pub fn validate_amount(amount: i128) -> Result<(), Error> {
    shared::validation::validate_positive_amount(amount).map_err(|_| Error::InvalidAmount)
}

/// Validates that `asset` is one StellarSpend currently supports, reusing
/// the shared allowlist rather than each contract inventing its own.
pub fn validate_asset(asset: &Symbol) -> Result<(), Error> {
    if shared::assets::is_supported_asset(asset.clone()) {
        Ok(())
    } else {
        Err(Error::UnsupportedAsset)
    }
}
