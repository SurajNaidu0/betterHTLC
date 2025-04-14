mod dailyvault;
use dailyvault::contract::{HTLC,RedeemConfig};
use bitcoin::{OutPoint,Txid,Amount,Address,Network};
use std::str::FromStr;
fn main() {
    let mut htlc = HTLC{
        htlc_funded_utxo: None,
        redeem_address: None,
        redeem_config: None
    };

    let prevout_txid = Txid::from_str("c49c613c390813075a3c7b9bcffba17e8d6468038342285176b3a138f68fa66f").unwrap();

    println!("prevout_txid: {}", prevout_txid);

    let htlc_outpoint = OutPoint::new(prevout_txid, 0);
    println!("htlc_outpoint: {}", htlc_outpoint);

    let amount = Amount::from_sat(100000);

    htlc.set_funded_htlc(htlc_outpoint, amount);

    htlc.redeem_address =  Some(Address::from_str("tb1p2fak0jfutw2ah7y568jv3hxvaz9aewpksnn26ewn94ygsrrtryjqv9c3c9").unwrap()
    .require_network(Network::Signet).unwrap());

    htlc.redeem_config = Some(RedeemConfig { payment_hash: "6644fd23b8327a04d86bdadbeba6903c1e9bfef68f9c9ee7c00cc8f59529430c".to_string(), preimage:"6e0d8625db81003c347c5ccc2c26f3a6b3cc8991c05624b9eeb80b1357ca8408".to_string()});

    let txn =htlc.create_redeem_tx().unwrap();

}


