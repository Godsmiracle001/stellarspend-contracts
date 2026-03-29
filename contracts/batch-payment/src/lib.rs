#![no_std]

mod test;
mod types;

use crate::types::Payment;
use soroban_sdk::{contract, contractimpl, symbol_short, token, Address, Env, Vec};

#[contract]
pub struct BatchPaymentContract;

#[contractimpl]
impl BatchPaymentContract {
    /// Transfers tokens from the caller to multiple recipients.
    ///
    /// # Arguments
    /// * `env` - The contract environment.
    /// * `from` - The address sending the tokens (must authorize the call).
    /// * `token` - The address of the token contract (e.g., USDC).
    /// * `payments` - A vector of `Payment` structs containing recipients and amounts.
    pub fn batch_transfer(env: Env, from: Address, token: Address, payments: Vec<Payment>) {
        // Require authorization from the sender
        from.require_auth();

        let token_client = token::Client::new(&env, &token);

        let mut total_amount: i128 = 0;
        let mut count: u32 = 0;

        // Generate a pseudo-unique batch ID based on ledger and timestamp (just for event tracking)
        let batch_id = env.ledger().sequence() as u64; // Simple ID for now

        for payment in payments.iter() {
            // Validation
            if payment.amount <= 0 {
                panic!("Payment amount must be positive");
            }

            // Execute transfer
            token_client.transfer(&from, &payment.recipient, &payment.amount);

            total_amount += payment.amount;
            count += 1;

            // Emit per-payment event
            // Topics: (payment, batch_id, recipient)
            // Data: (token, amount)
            let topics = (
                symbol_short!("payment"),
                batch_id,
                payment.recipient.clone(),
            );
            env.events()
                .publish(topics, (token.clone(), payment.amount));
        }

        // Emit batch completion event
        // Topics: (batch, complete, batch_id)
        // Data: (total_payments, total_amount)
        let topics = (symbol_short!("batch"), symbol_short!("complete"), batch_id);
        env.events().publish(topics, (count, total_amount));
    }
}
