#[macro_use]
extern crate afl;
extern crate abci;
extern crate chain_abci;
extern crate chain_core;
extern crate kvdb;
extern crate kvdb_memorydb;
extern crate secp256k1;
extern crate protobuf;
extern crate hex;


use abci::{Application, RequestCheckTx, RequestInitChain};
use chain_abci::app::ChainNodeApp;
use chain_abci::storage::{Storage, NUM_COLUMNS};
use chain_core::common::MerkleTree;
use chain_core::compute_app_hash;
use chain_core::init::{address::RedeemAddress, coin::Coin, config::InitConfig};
use chain_core::tx::fee::{LinearFee, Milli};
use chain_core::state::account::*;
use chain_core::tx::witness::TxInWitness;
use chain_core::tx::{
    data::{
        access::{TxAccess, TxAccessPolicy},
        address::ExtendedAddr,
        attribute::TxAttributes,
        input::TxoPointer,
        output::TxOut,
        Tx, TxId,
    },
    TxAux,
};
use kvdb::KeyValueDB;
use kvdb_memorydb::create;
use secp256k1::{
    key::{PublicKey, SecretKey},
    Message, Secp256k1, Signing,
};
use std::collections::BTreeMap;
use std::sync::Arc;
use chain_core::tx::TransactionId;
use chain_abci::storage::account::AccountWrapper;
use chain_abci::storage::tx::StarlingFixedKey;
use chain_core::init::config::{InitialValidator, ValidatorKeyType};
use chain_core::init::config::InitNetworkParameters;
use chain_core::init::config::AccountType;
use chain_core::tx::witness::EcdsaSignature;
use chain_abci::storage::account::AccountStorage;
use std::cell::RefCell;

fn create_db() -> Arc<dyn KeyValueDB> {
    Arc::new(create(NUM_COLUMNS.unwrap()))
}

fn create_account_db() -> AccountStorage {
    AccountStorage::new(Storage::new_db(Arc::new(create(1))), 20).expect("account db")
}

const TEST_CHAIN_ID: &str = "test-00";

pub fn get_ecdsa_witness<C: Signing>(
    secp: &Secp256k1<C>,
    txid: &TxId,
    secret_key: &SecretKey,
) -> EcdsaSignature {
    let message = Message::from_slice(&txid[..]).expect("32 bytes");
    let sig = secp.sign_recoverable(&message, &secret_key);
    return sig;
}

fn init_chain() -> ChainNodeApp {
    let address = "0x0e7c045110b8dbf29765047380898919c5dc56f4"
        .parse::<RedeemAddress>()
        .unwrap();
    let db = create_db();
    let total = (Coin::max() - Coin::unit()).unwrap();
    let validator_addr = "0x0e7c045110b8dbf29765047380898919c5cc56f4"
        .parse::<RedeemAddress>()
        .unwrap();

    let distribution: BTreeMap<RedeemAddress, (Coin, AccountType)> = [
        (address, (total, AccountType::ExternallyOwnedAccount)),
        (
            validator_addr,
            (Coin::unit(), AccountType::ExternallyOwnedAccount),
        ),
        (
            RedeemAddress::default(),
            (Coin::zero(), AccountType::Contract),
        ),
    ]
    .iter()
    .cloned()
    .collect();
    let params = InitNetworkParameters {
        initial_fee_policy: LinearFee::new(Milli::new(1, 1), Milli::new(1, 1)),
        required_council_node_stake: Coin::unit(),
        unbonding_period: 1,
    };
    let c = InitConfig::new(
        distribution,
        RedeemAddress::default(),
        RedeemAddress::default(),
        RedeemAddress::default(),
        params,
        vec![InitialValidator {
            staking_account_address: validator_addr,
            consensus_pubkey_type: ValidatorKeyType::Ed25519,
            consensus_pubkey_b64: "MDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDA=".to_string(),
        }],
    );
    let t = ::protobuf::well_known_types::Timestamp::new();
    let result = c.validate_config_get_genesis(t.get_seconds());
    if let Ok((accounts, rp, _nodes)) = result {
        let tx_tree = MerkleTree::empty();
        let mut account_tree =
            AccountStorage::new(Storage::new_db(Arc::new(create(1))), 20).expect("account db");

        let keys: Vec<StarlingFixedKey> = accounts.iter().map(|x| x.key()).collect();
        // TODO: get rid of the extra allocations
        let wrapped: Vec<AccountWrapper> =
            accounts.iter().map(|x| AccountWrapper(x.clone())).collect();
        let new_account_root = account_tree
            .insert(
                None,
                &mut keys.iter().collect::<Vec<_>>(),
                &mut wrapped.iter().collect::<Vec<_>>(),
            )
            .expect("initial insert");

        let genesis_app_hash = compute_app_hash(&tx_tree, &new_account_root, &rp);

        let example_hash = hex::encode_upper(genesis_app_hash);
        let mut app = ChainNodeApp::new_with_storage(
            &example_hash,
            TEST_CHAIN_ID,
            Storage::new_db(db.clone()),
            create_account_db(),
        );
        let mut req = RequestInitChain::default();
        req.set_time(t);
        req.set_app_state_bytes(serde_json::to_vec(&c).unwrap());
        req.set_chain_id(String::from(TEST_CHAIN_ID));
        app.init_chain(&req);
        app
    } else {
        panic!("distribution validation error: {}", result.err().unwrap());
    }
}

thread_local!(static APP: RefCell<ChainNodeApp> = RefCell::new(init_chain()));

fn fuzz_checktx(data: &[u8]) {
    let mut app = init_chain();
    let mut creq = RequestCheckTx::default();
    creq.set_tx(data.to_vec());
    app.check_tx(&creq);    
}

fn main() {
    fuzz!(|data: &[u8]| {
        fuzz_checktx(data);
    });
}