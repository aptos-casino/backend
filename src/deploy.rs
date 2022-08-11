// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_dice_backend::{Account, FaucetClient, FAUCET_URL, TESTNET_URL};
use aptos_dice_backend::HelloBlockchainClient;
use std::env;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    assert_eq!(
        args.len(),
        3,
        "\nExample:\ndeploy <private key hex> <path to compiled module.mv>\n"
    );

    let client = HelloBlockchainClient::new(TESTNET_URL.to_string());

    let privateKeyBytes = hex::decode(args.get(1).unwrap().as_bytes().as_ref()).unwrap();
    let mut ownerAccount = Account::new(Some(privateKeyBytes));

    println!("Owner: 0x{}", ownerAccount.address());

    println!("\nUpdate the module with Owner address, build, copy to the provided path, and press enter.");
    match std::io::stdin().read_line(&mut String::new()) {
        Ok(_n) => {}
        Err(error) => println!("error: {}", error),
    }

    let faucet_client = FaucetClient::new(FAUCET_URL.to_string(), client.rest_client.clone());
    faucet_client.fund_account(&ownerAccount.auth_key(), 10_000_000);

    let module_path = args.get(2).unwrap();
    let module_hex = hex::encode(std::fs::read(module_path).unwrap());

    println!("Publishing...");
    let mut tx_hash = client.publish_module(&mut ownerAccount, &module_hex);
    client.rest_client.wait_for_transaction(&tx_hash);
    println!("tx {}",tx_hash.to_string());

    println!("Initialize...");
    let contractAddress = ownerAccount.address();
    let mut tx_hash = client.initialize(&contractAddress, &mut ownerAccount);
    client.rest_client.wait_for_transaction(&tx_hash);
    println!("tx {}",tx_hash.to_string());
}
