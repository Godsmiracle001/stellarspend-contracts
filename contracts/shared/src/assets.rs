use soroban_sdk::Symbol;
/// Returns whether an asset symbol is supported by StellarSpend.
pub fn is_supported_asset(asset: Symbol) -> bool {
    asset == Symbol::new(&soroban_sdk::Env::default(), "XLM")
        || asset == Symbol::new(&soroban_sdk::Env::default(), "USDC")
        || asset == Symbol::new(&soroban_sdk::Env::default(), "EURC")
}
