#[test]
fn test_default_fee() {
    let env = Env::default();
    let config = FeeConfig {
        default_fee_rate: 100, // 1%
        windows: vec![],
    };
    let fee = calculate_fee(&env, 1000, &config);
    assert_eq!(fee, 10);
}

#[test]
fn test_promotional_window() {
    let env = Env::default();
    let now = env.ledger().timestamp();
    let config = FeeConfig {
        default_fee_rate: 100,
        windows: vec![FeeWindow { start: now - 10, end: now + 10, fee_rate: 50 }],
    };
    let fee = calculate_fee(&env, 1000, &config);
    assert_eq!(fee, 5);
}
