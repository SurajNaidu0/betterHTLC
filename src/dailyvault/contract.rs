use anyhow::{anyhow, Result};
use bitcoin::absolute::LockTime;
use bitcoin::consensus::Encodable;
use bitcoin::hashes::{sha256, Hash};
use bitcoin::hex::{Case, DisplayHex};
use bitcoin::key::{Secp256k1, Keypair};
use bitcoin::secp256k1::{ThirtyTwoByteHash, rand, Message};
use bitcoin::taproot::{LeafVersion, TaprootBuilder, TaprootSpendInfo, Signature};
use bitcoin::transaction::Version;
use bitcoin::consensus::encode::serialize;
use bitcoin::{
    amount, Address, Amount, Network, OutPoint, ScriptBuf, Sequence, TapLeafHash, TapSighashType, Transaction, TxIn, TxOut, Witness, XOnlyPublicKey
};
use bitcoin::sighash::{SighashCache, Prevouts};
use bitcoincore_rpc::jsonrpc::serde_json;
use log::{debug, info};
use secp256kfun::marker::{EvenY, NonZero, Public};
use secp256kfun::{Point, G};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::dailyvault::scripts::{
     htlc_redeem_script,htlc_instant_refund,htlc_refund_script
};
use crate::dailyvault::signature_building;
use crate::dailyvault::signature_building::{get_sigmsg_components, TxCommitmentSpec};


pub(crate) struct HTLC {
    pub htlc_funded_utxo: Option<HtlcFunded>,
    pub redeem_address: Option<Address>,
    pub redeem_config: Option<RedeemConfig>
}

pub struct RedeemConfig {
    pub payment_hash: String,
    pub preimage : String,
}
pub struct HtlcFunded{
    pub htlc_outpoint: OutPoint,
    pub amount: Amount,
}

impl HTLC {
    //add htlc outpoint (txid,vout)
    pub(crate) fn set_funded_htlc(&mut self, outpoint: OutPoint,amount: Amount) {
        self.htlc_funded_utxo = Some(HtlcFunded{
            htlc_outpoint: outpoint,
            amount: amount,
        });
    }

    //add redeem address
    pub(crate) fn set_redeem_address(&mut self, address: Address) {
        self.redeem_address = Some(address);
    }

    pub fn taproot_spend_info(&self)-> Result<TaprootSpendInfo> {
        let hash = sha256::Hash::hash(G.to_bytes_uncompressed().as_slice());
        let point: Point<EvenY, Public, NonZero> = Point::from_xonly_bytes(hash.into_32())
            .ok_or(anyhow!("G_X hash should be a valid x-only point"))?;
        let nums_key = XOnlyPublicKey::from_slice(point.to_xonly_bytes().as_slice())?;
        let secp = Secp256k1::new();
        let payment_hash = self.redeem_config.as_ref().unwrap().payment_hash.as_str();
        Ok(TaprootBuilder::new()
            .add_leaf(1, htlc_redeem_script(&self.redeem_address.as_ref().unwrap(),payment_hash))?
            .add_leaf(1, htlc_refund_script())?
            .finalize(&secp, nums_key)
            .expect("finalizing taproot spend info with a NUMS point should always work"))
    }
    //get the taproot address
    pub(crate) fn address(&self,network:Network) -> Result<Address> {
        let spend_info = self.taproot_spend_info()?;
        Ok(Address::p2tr_tweaked(spend_info.output_key(), network))
    }

    // pub(crate) fn create_trigger_tx
    pub(crate) fn create_redeem_tx(
        &self
    )->Result<Transaction>{
        let mut htlc_txin = TxIn {
            previous_output: self.htlc_funded_utxo.as_ref().unwrap().htlc_outpoint,
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        };

        //infinal vertion fee should be a input 
        let mut htlc_output = TxOut {
            script_pubkey: self.redeem_address.as_ref().unwrap().script_pubkey(),
            value: self.htlc_funded_utxo.as_ref().unwrap().amount ,
        };

        println!("htlc_output: {:?}", htlc_output);

        let mut htlc_tx = Transaction {
            version: Version(2),
            lock_time: LockTime::ZERO,
            input: vec![htlc_txin.clone()],
            output: vec![htlc_output],
        };

        println!("htlc_tx: {:?}", htlc_tx);

        let tx_commitment_spec = TxCommitmentSpec {
            ..Default::default()
        };

        let leaf_hash =
            TapLeafHash::from_script(&htlc_redeem_script(
                &self.redeem_address.as_ref().unwrap(),
                self.redeem_config.as_ref().unwrap().payment_hash.as_str(),
            ),LeafVersion::TapScript);

        let htlc_txout = TxOut {
                script_pubkey: self.redeem_address.as_ref().unwrap().script_pubkey(),
                value: self.htlc_funded_utxo.as_ref().unwrap().amount,
            };
        
        let contract_components = signature_building::grind_transaction(
                htlc_tx,
                signature_building::GrindField::LockTime,
                &[htlc_txout.clone()],
                leaf_hash,
            )?;
        
        let mut grinded_txn = contract_components.transaction;
        let witness_components = get_sigmsg_components(
                &tx_commitment_spec,
                &grinded_txn,
                0,
                &[htlc_txout.clone()],
                None,
                leaf_hash,
                TapSighashType::SinglePlusAnyoneCanPay,
            )?;

        for component in witness_components.iter() {
                debug!(
                    "pushing component <0x{}> into the witness",
                    component.to_hex_string(Case::Lower)
                );
                htlc_txin.witness.push(component.as_slice());
            }

        let computed_signature = signature_building::compute_signature_from_components(
                &contract_components.signature_components,
            )?;
        
        println!("computed_signature: {:?}", computed_signature);
        let mangled_signature: [u8; 63] = computed_signature[0..63].try_into().unwrap(); // chop
        println!("magled_signature: {:?}", mangled_signature);
        htlc_txin.witness.push(mangled_signature);
        htlc_txin.witness.push([computed_signature[63]]);
        htlc_txin.witness.push([computed_signature[63] + 1]); 

        htlc_txin
            .witness
            .push(htlc_redeem_script(
                &self.redeem_address.as_ref().unwrap(),
                self.redeem_config.as_ref().unwrap().payment_hash.as_str(),
            ).as_bytes());

        htlc_txin.witness.push(
                self.taproot_spend_info()?
                    .control_block(&(htlc_redeem_script(
                        &self.redeem_address.as_ref().unwrap(),
                        self.redeem_config.as_ref().unwrap().payment_hash.as_str()).clone(), LeafVersion::TapScript))
                    .expect("control block should work")
                    .serialize(),
            );

        grinded_txn.input.first_mut().unwrap().witness = htlc_txin.witness.clone();
        let raw_tx_hex = hex::encode(serialize(&grinded_txn));
        println!("Raw transaction hex: {}", raw_tx_hex);

        println!("htlc_tx: {:?}", grinded_txn);
        Ok(grinded_txn)
    }

    
}





