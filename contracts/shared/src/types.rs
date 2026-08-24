use soroban_sdk::contracttype;
/// A tier used by fee and reward curves.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Tier {
    pub threshold: i128,
    pub rate: i128,
}
