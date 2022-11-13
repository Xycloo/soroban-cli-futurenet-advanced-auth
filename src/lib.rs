#![no_std]
use soroban_auth::{verify, Ed25519Signature, Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, symbol, BigInt, Bytes, Env};

pub struct ExampleContract;

#[contracttype]
pub enum DataKey {
    Nonce(Identifier),
}

#[contractimpl]
impl ExampleContract {
    pub fn test_sig(e: Env, ed_sig: Ed25519Signature, key: Bytes, val: Bytes) {
        let sig = Signature::Ed25519(ed_sig);
        let nonce = get_nonce(&e, sig.identifier(&e));
        verify(&e, &sig, symbol!("change"), (key, val, nonce.clone()));
        e.data().set(DataKey::Nonce(sig.identifier(&e)), nonce + 1)
    }

    pub fn get(e: Env, key: Bytes) -> Identifier {
        e.data()
            .get(key)
            .unwrap_or_else(|| panic!("Key does not exist"))
            .unwrap()
    }

    pub fn nonce(e: Env, id: Identifier) -> BigInt {
        get_nonce(&e, id)
    }
}

fn get_nonce(e: &Env, id: Identifier) -> BigInt {
    e.data()
        .get(DataKey::Nonce(id))
        .unwrap_or_else(|| Ok(BigInt::zero(e)))
        .unwrap()
}

#[cfg(test)]
mod test;
