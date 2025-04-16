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
    htlc_redeem_script, htlc_refund_script
};
use crate::dailyvault::signature_building;
use crate::dailyvault::signature_building::{get_sigmsg_components, TxCommitmentSpec};

pub(crate) struct HTLC {
    pub htlc_funded_utxo: Option<HtlcFunded>,
    pub redeem_address: Option<Address>,
    pub redeem_config: Option<RedeemConfig>,
    pub refund_config: Option<RefundConfig>,
}

pub struct RefundConfig {
    pub refund_address: Address,
    pub refund_lock: i64,
}

pub struct RedeemConfig {
    pub payment_hash: String,
    pub preimage: String,
}

pub struct HtlcFunded {
    pub htlc_outpoint: OutPoint,
    pub amount: Amount,
}

impl HTLC {
    pub(crate) fn set_funded_htlc(&mut self, outpoint: OutPoint, amount: Amount) {
        self.htlc_funded_utxo = Some(HtlcFunded {
            htlc_outpoint: outpoint,
            amount: amount,
        });
    }

    pub(crate) fn set_redeem_address(&mut self, address: Address) {
        self.redeem_address = Some(address);
    }

    pub fn taproot_spend_info(&self) -> Result<TaprootSpendInfo> {
        let hash = sha256::Hash::hash(G.to_bytes_uncompressed().as_slice());
        let point: Point<EvenY, Public, NonZero> = Point::from_xonly_bytes(hash.into_32())
            .ok_or(anyhow!("G_X hash should be a valid x-only point"))?;
        let nums_key = XOnlyPublicKey::from_slice(point.to_xonly_bytes().as_slice())?;
        let secp = Secp256k1::new();
        let payment_hash = self.redeem_config.as_ref().unwrap().payment_hash.as_str();
        Ok(TaprootBuilder::new()
            .add_leaf(1, htlc_redeem_script(self.redeem_address.as_ref().unwrap(), payment_hash))?
            .add_leaf(1, htlc_refund_script(&self.refund_config.as_ref().unwrap().refund_address, &self.refund_config.as_ref().unwrap().refund_lock))?
            .finalize(&secp, nums_key)
            .expect("finalizing taproot spend info with a NUMS point should always work"))
    }

    pub(crate) fn address(&self, network: Network) -> Result<Address> {
        let spend_info = self.taproot_spend_info()?;
        Ok(Address::p2tr_tweaked(spend_info.output_key(), network))
    }

    pub(crate) fn create_redeem_tx(&self) -> Result<Transaction> {
        // Validate required fields
        if self.htlc_funded_utxo.is_none() || self.redeem_address.is_none() || self.redeem_config.is_none() {
            return Err(anyhow!("Missing required fields for redeem transaction"));
        }

        // Extract values safely
        let htlc_funded = self.htlc_funded_utxo.as_ref().unwrap();
        let redeem_address = self.redeem_address.as_ref().unwrap();
        let redeem_config = self.redeem_config.as_ref().unwrap();

        // Compute Taproot spend info once
        let spend_info = self.taproot_spend_info()?;

        // Create redeem script and leaf hash
        let redeem_script = htlc_redeem_script(redeem_address, &redeem_config.payment_hash);
        let leaf_hash = TapLeafHash::from_script(&redeem_script, LeafVersion::TapScript);

        // Define the previous HTLC output (to be spent)
        let htlc_address = self.address(Network::Bitcoin)?; // Assuming Bitcoin network
        let htlc_txout = TxOut {
            script_pubkey: htlc_address.script_pubkey(),
            value: htlc_funded.amount,
        };

        // Create transaction input
        let htlc_txin = TxIn {
            previous_output: htlc_funded.htlc_outpoint,
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        };

        // Create transaction output
        let htlc_output = TxOut {
            script_pubkey: redeem_address.script_pubkey(),
            value: htlc_funded.amount,
        };

        // Construct initial transaction
        let htlc_tx = Transaction {
            version: Version(2),
            lock_time: LockTime::ZERO,
            input: vec![htlc_txin],
            output: vec![htlc_output],
        };

        // Grind the transaction
        let tx_commitment_spec = TxCommitmentSpec {
            ..Default::default()
        };
        let contract_components = signature_building::grind_transaction(
            htlc_tx,
            signature_building::GrindField::LockTime,
            &[htlc_txout.clone()],
            leaf_hash,
        )?;
        let mut grinded_txn = contract_components.transaction;

        // Build and set the witness
        let witness = self.build_redeem_witness(
            &grinded_txn,
            0,
            &[htlc_txout],
            leaf_hash,
            &redeem_script,
            &spend_info,
            &tx_commitment_spec,
            &contract_components,
        )?;
        grinded_txn.input[0].witness = witness;

        // Serialize and print the raw transaction for debugging
        let raw_tx_hex = hex::encode(serialize(&grinded_txn));
        println!("Raw transaction hex: {}", raw_tx_hex);

        Ok(grinded_txn)
    }

    fn build_redeem_witness(
        &self,
        grinded_txn: &Transaction,
        input_index: usize,
        prevouts: &[TxOut],
        leaf_hash: TapLeafHash,
        redeem_script: &ScriptBuf,
        spend_info: &TaprootSpendInfo,
        tx_commitment_spec: &TxCommitmentSpec,
        contract_components: &signature_building::ContractComponents,
    ) -> Result<Witness> {
        // Compute witness components
        let witness_components = get_sigmsg_components(
            tx_commitment_spec,
            grinded_txn,
            input_index,
            prevouts,
            None,
            leaf_hash,
            TapSighashType::SinglePlusAnyoneCanPay,
        )?;

        let mut witness = Witness::new();

        // Push witness components
        for component in witness_components.iter() {
            debug!(
                "pushing component <0x{}> into the witness",
                component.to_hex_string(Case::Lower)
            );
            witness.push(component.as_slice());
        }

        // Compute and mangle signature
        let computed_signature = signature_building::compute_signature_from_components(
            &contract_components.signature_components,
        )?;
        let mangled_signature: [u8; 63] = computed_signature[0..63].try_into().unwrap();
        witness.push(mangled_signature);
        witness.push([computed_signature[63]]);
        witness.push([computed_signature[63] + 1]);

        // Push redeem script and control block
        witness.push(redeem_script.as_bytes());
        let control_block = spend_info
            .control_block(&(redeem_script.clone(), LeafVersion::TapScript))
            .expect("control block should work");
        witness.push(control_block.serialize());

        Ok(witness)
    }
}