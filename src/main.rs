mod dailyvault;
use dailyvault::contract::{RedeemConfig, RefundConfig, HTLC};
use bitcoin::{OutPoint,Txid,Amount,Address,Network};
use std::str::FromStr;
fn main() {
    let mut htlc = HTLC{
        htlc_funded_utxo: None,
        redeem_address: None,
        redeem_config: None,
        refund_config: None,
    };

    let prevout_txid = Txid::from_str("c49c613c390813075a3c7b9bcffba17e8d6468038342285176b3a138f68fa66f").unwrap();

    let htlc_outpoint = OutPoint::new(prevout_txid, 0);

    let amount = Amount::from_sat(100000);

    htlc.set_funded_htlc(htlc_outpoint, amount);

    htlc.redeem_address =  Some(Address::from_str("tb1p2fak0jfutw2ah7y568jv3hxvaz9aewpksnn26ewn94ygsrrtryjqv9c3c9").unwrap()
    .require_network(Network::Signet).unwrap());

    println!("redeem_address: {:?}", htlc.redeem_address.as_ref().unwrap().script_pubkey());


    htlc.redeem_config = Some(RedeemConfig { payment_hash: "7d71c056feba9afeb8ee135b8c83695b1ecf948a96d24494592a5743c6779a57".to_string(), preimage:Some("6644fd23b8327a04d86bdadbeba6903c1e9bfef68f9c9ee7c00cc8f59529430c".to_string())});

    let refund_address = Address::from_str("tb1p2fak0jfutw2ah7y568jv3hxvaz9aewpksnn26ewn94ygsrrtryjqv9c3c9").unwrap()
    .require_network(Network::Signet).unwrap();

    println!("redeem_address: {:?}", htlc.redeem_address.as_ref().unwrap().script_pubkey());

    htlc.refund_config = Some(RefundConfig { refund_address: refund_address, refund_lock: 100 as i64 });

    

    // let txn =htlc.create_redeem_tx().unwrap();

    let txn = htlc.create_refund_tx().unwrap();
    let txn_weight = txn.weight().to_vbytes_ceil();
    println!("txn weight: {:?}", txn_weight);

    

}


