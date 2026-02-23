#![cfg(test)]

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Ledger},
    Address, Env,
};

#[path = "../contracts/transactions.rs"]
mod transactions;

use transactions::{TimelockedTx, TransactionsContract, TransactionsContractClient};

fn setup_test_contract() -> (Env, Address, TransactionsContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    // Ensure deterministic starting timestamp.
    env.ledger().set_timestamp(1_000);

    let contract_id = env.register(TransactionsContract, ());
    let client = TransactionsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    (env, admin, client)
}

#[test]
fn test_schedule_timelocked_transaction_stores_record_and_emits_event() {
    let (env, _admin, client) = setup_test_contract();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let amount: i128 = 500;
    let payload = symbol_short!("pay");
    let asset: Option<Address> = None;

    let execute_at = env.ledger().timestamp() + 60;

    let scheduled: TimelockedTx = client.schedule_timelocked_transaction(
        &from,
        &to,
        &amount,
        &payload,
        &asset,
        &execute_at,
    );

    assert_eq!(scheduled.from, from);
    assert_eq!(scheduled.to, to);
    assert_eq!(scheduled.amount, amount);
    assert_eq!(scheduled.execute_at, execute_at);
    assert_eq!(scheduled.executed, false);
    assert_eq!(scheduled.canceled, false);

    // Fetch via getter
    let fetched = client
        .get_timelocked_transaction(&scheduled.id)
        .expect("expected stored timelocked tx");
    assert_eq!(fetched.id, scheduled.id);
}

#[test]
#[should_panic]
fn test_cannot_schedule_with_past_or_current_timestamp() {
    let (env, _admin, client) = setup_test_contract();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let amount: i128 = 100;
    let payload = symbol_short!("pay");
    let asset: Option<Address> = None;

    let now = env.ledger().timestamp();

    // Using execute_at <= now should be rejected.
    client.schedule_timelocked_transaction(&from, &to, &amount, &payload, &asset, &now);
}

#[test]
#[should_panic]
fn test_cannot_execute_before_execute_at() {
    let (env, admin, client) = setup_test_contract();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let amount: i128 = 250;
    let payload = symbol_short!("pay");
    let asset: Option<Address> = None;

    let execute_at = env.ledger().timestamp() + 300;
    let scheduled = client.schedule_timelocked_transaction(
        &from,
        &to,
        &amount,
        &payload,
        &asset,
        &execute_at,
    );

    // Even the admin cannot execute before the scheduled time.
    client.execute_timelocked_transaction(&admin, &scheduled.id);
}

#[test]
fn test_execute_after_execute_at_moves_balance_and_marks_executed() {
    let (env, admin, client) = setup_test_contract();

    let from = Address::generate(&env);
    let to = Address::generate(&env);

    // Seed balance using existing admin-only setter.
    client.set_balance(&admin, &from, &1_000);

    let amount: i128 = 400;
    let payload = symbol_short!("pay");
    let asset: Option<Address> = None;

    let execute_at = env.ledger().timestamp() + 10;
    let scheduled = client.schedule_timelocked_transaction(
        &from,
        &to,
        &amount,
        &payload,
        &asset,
        &execute_at,
    );

    // Advance time to just after execute_at.
    env.ledger().set_timestamp(execute_at + 1);

    // Allow admin to execute on behalf of the user.
    client.execute_timelocked_transaction(&admin, &scheduled.id);

    let executed = client
        .get_timelocked_transaction(&scheduled.id)
        .expect("missing timelocked tx");
    assert!(executed.executed);
    assert!(!executed.canceled);

    // Balance should have moved.
    assert_eq!(client.get_balance(&from), 600);
    assert_eq!(client.get_balance(&to), 400);
}

#[test]
fn test_cancel_before_execution_prevents_later_execution() {
    let (env, admin, client) = setup_test_contract();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    client.set_balance(&admin, &from, &1_000);

    let amount: i128 = 200;
    let payload = symbol_short!("pay");
    let asset: Option<Address> = None;

    let execute_at = env.ledger().timestamp() + 100;
    let scheduled = client.schedule_timelocked_transaction(
        &from,
        &to,
        &amount,
        &payload,
        &asset,
        &execute_at,
    );

    // User cancels before execution window.
    client.cancel_timelocked_transaction(&from, &scheduled.id);

    let cancelled = client
        .get_timelocked_transaction(&scheduled.id)
        .expect("missing timelocked tx");
    assert!(cancelled.canceled);
    assert!(!cancelled.executed);

    // Move time forward and ensure execution now fails.
    env.ledger().set_timestamp(execute_at + 1);

    // Attempting to execute should panic due to AlreadyCanceled.
    let result = std::panic::catch_unwind(|| {
        client.execute_timelocked_transaction(&admin, &scheduled.id);
    });
    assert!(result.is_err());

    // Balances unchanged.
    assert_eq!(client.get_balance(&from), 1_000);
    assert_eq!(client.get_balance(&to), 0);
}

#[test]
fn test_only_owner_or_admin_can_cancel_or_execute() {
    let (env, admin, client) = setup_test_contract();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    client.set_balance(&admin, &from, &1_000);

    let amount: i128 = 100;
    let payload = symbol_short!("pay");
    let asset: Option<Address> = None;

    let execute_at = env.ledger().timestamp() + 50;
    let scheduled = client.schedule_timelocked_transaction(
        &from,
        &to,
        &amount,
        &payload,
        &asset,
        &execute_at,
    );

    let outsider = Address::generate(&env);

    // Outsider cannot cancel.
    let cancel_result = std::panic::catch_unwind(|| {
        client.cancel_timelocked_transaction(&outsider, &scheduled.id);
    });
    assert!(cancel_result.is_err());

    // Advance time and outsider cannot execute either.
    env.ledger().set_timestamp(execute_at + 1);
    let exec_result = std::panic::catch_unwind(|| {
        client.execute_timelocked_transaction(&outsider, &scheduled.id);
    });
    assert!(exec_result.is_err());
}

