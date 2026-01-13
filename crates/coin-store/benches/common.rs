use std::fs;
use std::collections::HashMap;

use coin_store::filter::UtxoFilter; 
use coin_store::store::Store;
use coin_store::executor::UtxoStore;

use simplicityhl::elements::hashes::Hash;
use simplicityhl::elements::{AssetId, OutPoint, TxOut, TxOutWitness, Txid, Script, AddressParams, Transaction};
use simplicityhl::elements::confidential::{Asset, Nonce, Value};
use simplicityhl::simplicity::bitcoin::key::Keypair;
use simplicityhl::elements::secp256k1_zkp::{self as secp256k1, SecretKey, ZERO_TWEAK};
use simplicityhl::simplicity::bitcoin::secp256k1::Secp256k1;
use simplicityhl::elements::pset::PartiallySignedTransaction;


use simplicityhl_core::{LIQUID_TESTNET_BITCOIN_ASSET, LIQUID_TESTNET_TEST_ASSET_ID_STR};

use std::str::FromStr;
use contracts::sdk::taproot_pubkey_gen::{TaprootPubkeyGen, get_random_seed};
use contracts::sdk::build_option_creation;
use contracts::options::OptionsArguments;

/* 
pub async fn setup_db_default() -> (Store, Vec<UtxoFilter>, String) {
    let path = "/tmp/benchmark_stress.db";
    let _ = fs::remove_file(path);

    let store = Store::create(path).await.unwrap();

    let num_assets = 50;
    let utxos_per_asset = 100;
    let coin_value = 10;

    let mut heavy_filters = Vec::new();

    for i in 0..num_assets {
        let mut asset_bytes = [0u8; 32];
        asset_bytes[0] = (i % 255) as u8;
        asset_bytes[1] = (i / 255) as u8;
        let asset_id = AssetId::from_slice(&asset_bytes).unwrap();

        for j in 0..utxos_per_asset {
            let mut txid_bytes = [0u8; 32];
            txid_bytes[0] = (i % 255) as u8;
            txid_bytes[31] = (j % 255) as u8;
            txid_bytes[15] = (j / 255) as u8;

            let outpoint = OutPoint::new(Txid::from_byte_array(txid_bytes), j as u32);

            store.insert(
                outpoint,
                make_explicit_txout(asset_id, coin_value),
                None,
            ).await.unwrap();
        }
        
        heavy_filters.push(
            UtxoFilter::new().asset_id(asset_id).required_value(950)
        );
        heavy_filters.push(
            UtxoFilter::new().asset_id(asset_id).required_value(1500)
        );
    }

    (store, heavy_filters, path.to_string())
}

*/
fn setup_tx_and_contract(
        keypair: &Keypair,
        start_time: u32,
        expiry_time: u32,
        collateral_per_contract: u64,
        settlement_per_contract: u64,
    ) -> Result<(PartiallySignedTransaction, TaprootPubkeyGen), Box<dyn std::error::Error>> {
        let option_outpoint = OutPoint::new(Txid::from_slice(&[1; 32])?, 0);
        let grantor_outpoint = OutPoint::new(Txid::from_slice(&[2; 32])?, 0);

        let issuance_asset_entropy = get_random_seed();

        let option_arguments = OptionsArguments::new(
            start_time,
            expiry_time,
            collateral_per_contract,
            settlement_per_contract,
            *LIQUID_TESTNET_BITCOIN_ASSET,
            AssetId::from_str(LIQUID_TESTNET_TEST_ASSET_ID_STR)?,
            issuance_asset_entropy,
            (option_outpoint, false),
            (grantor_outpoint, false),
        );

        Ok(
            build_option_creation(
                &keypair.public_key(),
                (
                    option_outpoint,
                    TxOut {
                        asset: Asset::Explicit(*LIQUID_TESTNET_BITCOIN_ASSET),
                        value: Value::Explicit(500),
                        nonce: Nonce::Null,
                        script_pubkey: Script::new(),
                        witness: TxOutWitness::default(),
                    },
                ),
                (
                    grantor_outpoint,
                    TxOut {
                        asset: Asset::Explicit(*LIQUID_TESTNET_BITCOIN_ASSET),
                        value: Value::Explicit(1000),
                        nonce: Nonce::Null,
                        script_pubkey: Script::new(),
                        witness: TxOutWitness::default(),
                    },
                ),
                &option_arguments,
                issuance_asset_entropy,
                100,
                &AddressParams::LIQUID_TESTNET,
            )?,
        )
    }
    
pub async fn setup_db() -> (Store, Vec<UtxoFilter>, String) {
    let path = "/tmp/benchmark_stress.db";
    let _ = fs::remove_file(path);

    let mut filters_default = vec![];
    let store = Store::create(path).await.unwrap();

    for i in 1..10{
        let secp = Secp256k1::new();
        let keypair = Keypair::from_secret_key(
            &secp,
            &SecretKey::from_slice(&[1u8; 32]).unwrap(),
        );

        let (pst, _tpg) = setup_tx_and_contract(&keypair, 0, 0, 20, 25).unwrap();
        let tx: Transaction = pst.extract_tx().unwrap();

        let mut blinder_keys = HashMap::new();

        for (i, _output) in tx.output.iter().enumerate() {
            blinder_keys.insert(i, keypair); 
        }

        store.insert_transaction(&tx, blinder_keys).await.unwrap();

        let secret_key = keypair.secret_key();

        for output in tx.output.iter() {
            let asset_id = match output.asset {
                Asset::Explicit(id) => id,
                Asset::Confidential(_) => {
                    let unblinded = output.unblind(&secp, secret_key).expect("Failed to unblind in setup");
                    unblinded.asset
                }
                _ => panic!("Null asset?"),
            };
            filters_default.push(
                UtxoFilter::new().asset_id(asset_id).required_value(1)
            );
        }
    }

    (store, filters_default, path.to_string())
}