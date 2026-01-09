use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::fs;
use std::time::Duration;
use tokio::runtime::Runtime;

use coin_store::filter::{UtxoFilter}; 
use coin_store::store::{Store};
use coin_store::executor::{UtxoStore};

use simplicityhl::elements::hashes::{Hash};
use simplicityhl::elements::{AssetId, OutPoint, TxOut, TxOutWitness, Txid};
use simplicityhl::elements::confidential::{Asset, Nonce, Value};
use simplicityhl::elements::{Script};


fn make_explicit_txout(asset_id: AssetId, value: u64) -> TxOut {
    TxOut {
        asset: Asset::Explicit(asset_id),
        value: Value::Explicit(value),
        nonce: Nonce::Null,
        script_pubkey: Script::new(),
        witness: TxOutWitness::default(),
    }
}

async fn setup_db() -> (Store, Vec<UtxoFilter>, String) {
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

fn criterion_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let (store, filters, db_path) = rt.block_on(async {
        setup_db().await
    });

    let mut group = c.benchmark_group("UTXO Queries");
    
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("current_implementation", |b| {
        b.to_async(&rt).iter(|| async {
            store.query_utxos(black_box(&filters)).await.unwrap();
        })
    });

    /*
    group.bench_function("optimized_implementation", |b| {
        b.to_async(&rt).iter(|| async {
            store.optimized_query_utxos(black_box(&filters)).await.unwrap();
        })
    });
    */

    group.finish();

    let _ = fs::remove_file(db_path);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);