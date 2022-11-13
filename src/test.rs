use crate::{ExampleContract, ExampleContractClient};
use soroban_auth::Signature;
use soroban_sdk::{
    bytes, bytesn, symbol,
    testutils::{Ledger, LedgerInfo},
    BigInt, Env,
};

extern crate std;

#[test]
fn test_use_advanced_auth() {
    let e = Env::default();

    e.ledger().set(LedgerInfo {
        timestamp: 1668106305,
        protocol_version: 20,
        sequence_number: 10,
        network_passphrase: std::vec![
            83, 116, 97, 110, 100, 97, 108, 111, 110, 101, 32, 78, 101, 116, 119, 111, 114, 107,
            32, 59, 32, 70, 101, 98, 114, 117, 97, 114, 121, 32, 50, 48, 49, 55,
        ],
        base_reserve: 10,
    });

    let (user_1_id, user_1_sign) = soroban_auth::testutils::ed25519::generate(&e);

    let contract_id = e.register_contract(
        &std::option::Option::Some(
            bytesn!(&e, 0x69f7e580340b3f963e56a40a11a4bc89264b53583fc92e65ef44efd051ab5a9b),
        ),
        ExampleContract,
    );
    let client = ExampleContractClient::new(&e, &contract_id);

    let nonce = BigInt::from_u32(&e, 0);
    let sig = soroban_auth::testutils::ed25519::sign(
        &e,
        &user_1_sign,
        &contract_id,
        symbol!("change"),
        (bytes!(&e, 0x7), bytes!(&e, 0x7), nonce),
    );

    //    std::println!("{:?}", sig);

    let sig_obj = match sig {
        Signature::Ed25519(obj) => obj,
        _ => panic!("not ed25519"),
    };

    client.test_sig(&sig_obj, &bytes!(&e, 0x7), &bytes!(&e, 0x7));
}
