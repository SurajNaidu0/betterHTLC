mod htlc;
mod wallet;
mod settings;
use htlc::contract::{RedeemConfig, RefundConfig, HTLC};
use bitcoin::{locktime, Address, Amount, Network, OutPoint, Transaction, Txid};
use std::str::FromStr;
use anyhow::Result;
use crate::settings::Settings;
use clap::Parser;
use std::path::PathBuf;
use log::{debug, error, info};
use crate::wallet::Wallet;
use crate::htlc::contract::{HtlcFunded,add_fee_to_txn,compute_taproot_sighash};
use bitcoin::consensus::Encodable;
use bitcoincore_rpc::{RawTx, RpcApi};

// #[derive(Parser)]
// struct Cli {
//     #[arg(short, long, default_value = "settings.toml")]
//     settings_file: PathBuf,

//     #[command(subcommand)]
//     action: Action,
// }

// #[derive(Parser)]

// //TODO: add redeem, refund, redeem steal, refund steal actions
// enum Action {
//     Deposit{refund_address:String,redeem_address:String,payment_hash:String},
//     AdHokTesting,
// }

// fn main() -> Result<()> {
//     env_logger::init();
//     println!("Htlc using OP_CAT");

//     let args = Cli::parse();
    
//     let settings = match Settings::from_toml_file(&args.settings_file) {
//         Ok(settings) => settings,
//         Err(e) => {
//             error!("Error reading settings file: {}", e);
//             info!(
//                 "Creating a new settings file at {}",
//                 args.settings_file.display()
//             );
//             let settings = Settings::default();
//             settings.to_toml_file(&args.settings_file)?;
//             settings
//         }
//     };
//     match args.action {
//         Action::Deposit{refund_address,redeem_address,payment_hash} => deposit(&refund_address,&redeem_address,100 as i64,&payment_hash,&settings)?,
//         Action::AdHokTesting => ad_hoc_testing(&settings)?,
//     };
//     Ok(())
// }

// fn deposit(refund_address:&str,redeem_address:&str,locktime:i64,payment_hash:&str,settings: &Settings)-> Result<()> {
//     let miner_wallet = Wallet::new("miner", &settings);
//     while miner_wallet.get_balance()? < Amount::from_btc(1.0f64)? {
//         debug!("Mining some blocks to get some coins");
//         miner_wallet.mine_blocks(Some(1))?;
//     };

//     println!("Miner wallet balance: {:?}", miner_wallet.get_new_address()?);

//     println!("Making htlc contract");

//     let redeem_address = Address::from_str(redeem_address)?.require_network(settings.network)?;

//     let refund_address = Address::from_str(refund_address)?.require_network(settings.network)?;

//     let redeem_config = RedeemConfig {
//         payment_hash: payment_hash.to_string(),
//         preimage: None,
//     };

//     let refund_config = RefundConfig {
//         refund_address: refund_address,
//         refund_lock: locktime,
//     };
//     let htlc_contract = HTLC {
//         htlc_funded_utxo: None,
//         redeem_address: Some(redeem_address),
//         redeem_config: Some(redeem_config),
//         refund_config: Some(refund_config),
//     };
//     let htlc_address:Address = htlc_contract.address(settings.network)?;
//     println!("htlc address: {:?}", htlc_address);
//     let deposit_tx = miner_wallet.send(&htlc_address, Amount::from_sat(100_000_000))?;

//     let htlc_funded = HtlcFunded {
//         htlc_outpoint: deposit_tx,
//         amount: Amount::from_sat(100_000_000),
//     };
//     miner_wallet.mine_blocks(Some(1))?;
//     println!("Funding htlc contract {:?}",htlc_contract);
//     Ok(())
// }

// fn ad_hoc_testing(settings: &Settings)-> Result<()>{
//     let preimage = "6644fd23b8327a04d86bdadbeba6903c1e9bfef68f9c9ee7c00cc8f59529430c";
//     let payment_hash = "7d71c056feba9afeb8ee135b8c83695b1ecf948a96d24494592a5743c6779a57";
//     let locktime = 100 as i64;

//     let miner_wallet = Wallet::new("miner", &settings);
//     while miner_wallet.get_balance()? < Amount::from_btc(2.0f64)? {
//         debug!("Mining some blocks to get some coins");
//         miner_wallet.mine_blocks(Some(1))?;
//     };

//     // person who will deposit funds to htlc and redeem after timeout 
//     let initializer_wallet = Wallet::new("initializer", &settings);

//     // minner sending funds to initializer wallet
//     let initializer_address = initializer_wallet.get_new_address()?;
//     let deposit_tx = miner_wallet.send(&initializer_address, Amount::from_sat(200_000_000))?;
//     miner_wallet.mine_blocks(Some(1))?; // confirm the transfer to initializer wallet
//     println!("initializer address balance: {:?}", initializer_wallet.get_balance()?);

//     // person who will redeem from the htlc 
//     let redeem_wallet = Wallet::new("redeem", &settings);

//     //setting redeem config
//     let redeem_config = RedeemConfig {
//         payment_hash: payment_hash.to_string(),
//         preimage: Some(preimage.to_string()),
//     };

//     //setting refund config
//     let refund_address = redeem_wallet.get_new_address()?;
//     let refund_config = RefundConfig {
//         refund_address: refund_address,
//         refund_lock: locktime,
//     };

//     //creating htlc contract
//     let mut htlc_contract = HTLC {
//         htlc_funded_utxo: None,
//         redeem_address: Some(initializer_address),
//         redeem_config: Some(redeem_config),
//         refund_config: Some(refund_config),
//     };

//     //creating htlc address
//     let htlc_address:Address = htlc_contract.address(settings.network)?;
//     println!("htlc address: {:?}", htlc_address);

//     //sending funds to htlc address
//     let deposit_tx = initializer_wallet.send(&htlc_address, Amount::from_sat(100_000_000))?;
//     miner_wallet.mine_blocks(Some(1))?; // confirm the deposit transaction

//     //checking the initializer balance 
//     println!("initializer balance: {:?}", initializer_wallet.get_balance()?);

//     //htlc funded info
//     let htlc_funded = HtlcFunded {
//         htlc_outpoint: deposit_tx,
//         amount: Amount::from_sat(100_000_000),
//     };
//     htlc_contract.htlc_funded_utxo = Some(htlc_funded);

//     //creating redeem transaction
//     let mut redeem_tx:Transaction = htlc_contract.create_redeem_tx()?;

//     //fee wallet
//     let fee_wallet = Wallet::new("fee", &settings);
//     let fee_address = fee_wallet.get_new_address()?;
//     let deposit_tx = miner_wallet.send(&fee_address, Amount::from_sat(10000))?;
//     miner_wallet.mine_blocks(Some(1))?; // confirm the transfer to fee wallet
//     println!("fee address balance: {:?}", fee_wallet.get_balance()?);

//     //adding fee to the redeem transaction
//     let fee_utxo_value = Amount::from_sat(10000);
//     let fee_sats = Amount::from_sat(1000);
//     let mut fee_tx: &mut Transaction = add_fee_to_txn(&mut redeem_tx, deposit_tx, fee_utxo_value, fee_sats,fee_address)?;
//     let signed_tx = fee_wallet.sign_tx(&fee_tx)?;
//     let mut serialized_tx = Vec::new();
//     signed_tx.consensus_encode(&mut serialized_tx).unwrap();
//     println!("serialized tx: {:?}", serialized_tx.raw_hex());
//     let txid = fee_wallet.broadcast_tx(&serialized_tx, None)?;
//     println!("sent trigger transaction txid: {}", txid);
//     miner_wallet.mine_blocks(Some(1))?;



//     Ok(())
// }

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

    println!("htlc address: {}", htlc.address(Network::Signet).unwrap().script_pubkey());

    

    let fee_amount = Amount::from_sat(1000);
    let txn =htlc.create_redeem_tx_with_fee(fee_amount).unwrap();

    println!("Checking Signature Message Redeem check");
  
    let txn_weight = txn.weight().to_vbytes_ceil();
    println!("txn weight: {:?}", txn_weight);

    let txn = htlc.create_refund_tx().unwrap();
    let txn_weight = txn.weight().to_vbytes_ceil();
    println!("txn weight: {:?}", txn_weight);



}


