use crate::types::Tier;
/// Calculates the rate for the highest matching tier.
pub fn calculate_tiered_rate(value: i128, tiers: &[Tier]) -> i128 {
    tiers
        .iter()
        .filter(|t| value >= t.threshold)
        .map(|t| t.rate)
        .max()
        .unwrap_or(0)
}
