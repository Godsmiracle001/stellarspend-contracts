use soroban_sdk::{Env,Symbol,Val};
/// Emits a standard StellarSpend event.
pub fn emit_event(env:&Env,name:Symbol,data:Val){env.events().publish((name,),data)}
