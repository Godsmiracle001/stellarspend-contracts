use soroban_sdk::Vec;

use crate::types::PolicyRule;
use crate::Error;

/// Validates every rule in a policy set. `limit` and `zk_required_above` may
/// be zero (a legitimate "block this category entirely" / "always require a
/// proof" policy) but never negative.
pub fn validate_rules(rules: &Vec<PolicyRule>) -> Result<(), Error> {
    for rule in rules.iter() {
        if rule.limit < 0 || rule.zk_required_above < 0 {
            return Err(Error::InvalidAmount);
        }
    }
    Ok(())
}
