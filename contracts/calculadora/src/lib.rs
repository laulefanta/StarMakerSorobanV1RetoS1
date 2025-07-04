#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

const RESULTADO: Symbol = symbol_short!("RESULTADO");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn sumar(env: Env, a: i128, b: i128) -> i128 {
        let resultado = a + b;
        env.storage().persistent().set(&RESULTADO, &resultado);
        resultado
    }

    pub fn resultado_anterior(env: Env) -> i128 {
        match env.storage().persistent().get::<Symbol, i128>(&RESULTADO) {
            Some(r) => r,
            None => 0,
        }
    }
}

mod test;
