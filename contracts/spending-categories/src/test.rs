#[cfg(test)]
mod tests {
    use crate::{Contract, ContractClient, Error};
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address, Env, Symbol,
    };

    const DAY: u64 = 86_400;

    fn setup<'a>() -> (Env, ContractClient<'a>, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        client.initialize(&admin);
        (env, client, admin)
    }

    fn groceries(env: &Env) -> Symbol {
        Symbol::new(env, "Groceries")
    }

    fn daily(env: &Env) -> Symbol {
        Symbol::new(env, "daily")
    }

    // ── Happy path (the issue's own scenario) ───────────────────────────

    #[test]
    fn assign_retrieve_and_accumulate_category_total() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);
        let category = groceries(&env);

        client.set_category(&user, &1, &category);
        assert_eq!(client.get_category(&1), Some(category.clone()));

        client.record_category_spend(&user, &1, &30);
        assert_eq!(
            client.get_category_total(&user, &category, &daily(&env)),
            30
        );

        // A second transaction in the same category accumulates.
        client.set_category(&user, &2, &category);
        client.record_category_spend(&user, &2, &20);
        assert_eq!(
            client.get_category_total(&user, &category, &daily(&env)),
            50
        );
    }

    #[test]
    fn get_category_returns_none_for_an_untagged_transaction() {
        let (_, client, _admin) = setup();
        assert_eq!(client.get_category(&999), None);
    }

    // ── Unauthorized caller ──────────────────────────────────────────────

    #[test]
    fn set_category_requires_caller_authorization() {
        let env = Env::default();
        // No mock_all_auths — nothing has authorized anything.
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let user = Address::generate(&env);

        let result = client.try_set_category(&user, &1, &groceries(&env));
        assert!(result.is_err());
    }

    #[test]
    fn record_category_spend_rejects_a_caller_who_is_not_the_original_owner() {
        let (env, client, _admin) = setup();
        let owner = Address::generate(&env);
        let impostor = Address::generate(&env);
        client.set_category(&owner, &1, &groceries(&env));

        let result = client.try_record_category_spend(&impostor, &1, &10);
        assert_eq!(result, Err(Ok(Error::Unauthorized)));

        // The owner's total is untouched by the rejected attempt.
        assert_eq!(
            client.get_category_total(&owner, &groceries(&env), &daily(&env)),
            0
        );
    }

    // ── Boundary values ──────────────────────────────────────────────────

    #[test]
    fn set_category_rejects_recategorizing_an_already_tagged_transaction() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);
        client.set_category(&user, &1, &groceries(&env));

        let other = Symbol::new(&env, "Rent");
        let result = client.try_set_category(&user, &1, &other);
        assert_eq!(result, Err(Ok(Error::AlreadyCategorized)));
        // Original category unchanged.
        assert_eq!(client.get_category(&1), Some(groceries(&env)));
    }

    #[test]
    fn record_category_spend_rejects_non_positive_amount() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);
        client.set_category(&user, &1, &groceries(&env));

        assert_eq!(
            client.try_record_category_spend(&user, &1, &0),
            Err(Ok(Error::InvalidAmount))
        );
        assert_eq!(
            client.try_record_category_spend(&user, &1, &-5),
            Err(Ok(Error::InvalidAmount))
        );
    }

    #[test]
    fn record_category_spend_requires_the_transaction_to_be_categorized_first() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);
        let result = client.try_record_category_spend(&user, &42, &10);
        assert_eq!(result, Err(Ok(Error::CategoryNotSet)));
    }

    #[test]
    fn get_category_total_for_an_unknown_period_symbol_is_zero() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);
        client.set_category(&user, &1, &groceries(&env));
        client.record_category_spend(&user, &1, &30);

        let bogus = Symbol::new(&env, "fortnightly");
        assert_eq!(
            client.get_category_total(&user, &groceries(&env), &bogus),
            0
        );
    }

    // ── Period reset behavior ────────────────────────────────────────────

    #[test]
    fn category_total_resets_when_a_new_daily_period_begins() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);
        let category = groceries(&env);
        client.set_category(&user, &1, &category);
        client.record_category_spend(&user, &1, &30);
        assert_eq!(
            client.get_category_total(&user, &category, &daily(&env)),
            30
        );

        env.ledger().with_mut(|l| l.timestamp += DAY + 1);

        // New daily bucket starts empty even though the tx_id remains
        // categorized (categorization itself never expires).
        assert_eq!(client.get_category_total(&user, &category, &daily(&env)), 0);
        assert_eq!(client.get_category(&1), Some(category.clone()));

        client.set_category(&user, &2, &category);
        client.record_category_spend(&user, &2, &15);
        assert_eq!(
            client.get_category_total(&user, &category, &daily(&env)),
            15
        );
    }

    // ── Admin / initialize ───────────────────────────────────────────────

    #[test]
    fn initialize_twice_is_rejected() {
        let (_, client, admin) = setup();
        assert_eq!(
            client.try_initialize(&admin),
            Err(Ok(Error::AlreadyInitialized))
        );
    }
}
