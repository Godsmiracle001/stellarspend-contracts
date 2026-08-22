use crate::Error;

/// Validates a spend amount recorded against a category: must be strictly
/// positive. Reuses `shared::validation::validate_positive_amount`.
pub fn validate_amount(amount: i128) -> Result<(), Error> {
    shared::validation::validate_positive_amount(amount).map_err(|_| Error::InvalidAmount)
}
