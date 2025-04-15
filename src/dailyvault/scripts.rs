use crate::dailyvault::signature_building::{BIP0340_CHALLENGE_TAG, DUST_AMOUNT, G_X, TAPSIGHASH_TAG};
use bitcoin::opcodes::all::{
    OP_2DUP, OP_CAT, OP_CHECKSIG, OP_CSV, OP_DROP, OP_DUP, OP_EQUALVERIFY, OP_FROMALTSTACK,
    OP_HASH256, OP_ROT, OP_SHA256, OP_SWAP, OP_TOALTSTACK, OP_CHECKSIGVERIFY
};
use bitcoin::script::Builder;
use bitcoin::{Address, Script, ScriptBuf, Sequence, XOnlyPublicKey};
use bitcoin::blockdata::script::PushBytesBuf;

pub(crate) fn htlc_redeem_script(redeem_address:&Address,payment_hash:&str) -> ScriptBuf {
    let mut builder = Script::builder();
    builder = builder
    .push_opcode(OP_SHA256)
    .push_slice(PushBytesBuf::try_from(hex::decode(payment_hash).expect("Invalid secret hash hex")).unwrap())
    .push_opcode(OP_EQUALVERIFY)
    .push_opcode(OP_TOALTSTACK)
    .push_opcode(OP_TOALTSTACK)
    .push_opcode(OP_TOALTSTACK) 
    .push_slice(PushBytesBuf::try_from(hex::decode("0083020000000000000002").expect("Invalid secret hash hex")).unwrap())
    .push_opcode(OP_SWAP)
    .push_opcode(OP_CAT)
    .push_opcode(OP_SWAP)
    .push_opcode(OP_DUP) 
    .push_opcode(OP_TOALTSTACK)
    .push_opcode(OP_CAT)
    .push_opcode(OP_SWAP)
    .push_opcode(OP_CAT)
    .push_opcode(OP_FROMALTSTACK)
    // Push [length, script_pubkey_bytes...]
        .push_slice({
            let script_pubkey = redeem_address.script_pubkey(); // Store Script
            let script_bytes = script_pubkey.as_bytes(); // Borrow
            let length = script_bytes.len() as u8;
            if script_bytes.len() > 255 {
                panic!("ScriptPubKey too long: {} bytes", script_bytes.len());
            }
            let mut bytes_with_length = vec![length];
            bytes_with_length.extend_from_slice(script_bytes);
            PushBytesBuf::try_from(bytes_with_length).expect("Invalid scriptPubKey bytes")
        })
    .push_opcode(OP_CAT)
    .push_opcode(OP_SHA256)
    .push_opcode(OP_CAT)
    .push_opcode(OP_SWAP)
    .push_opcode(OP_CAT)
    .push_slice(*TAPSIGHASH_TAG) // push tag
        .push_opcode(OP_SHA256) // hash tag
        .push_opcode(OP_DUP) // dup hash
        .push_opcode(OP_ROT) // move the sighash to the top of the stack
        .push_opcode(OP_CAT)
        .push_opcode(OP_CAT)
        .push_opcode(OP_SHA256) 
        .push_slice(*BIP0340_CHALLENGE_TAG) 
        .push_opcode(OP_SHA256)
        .push_opcode(OP_DUP)
        .push_opcode(OP_ROT) // bring challenge to the top of the stack
        .push_slice(*G_X) // G is used for the pubkey and K
        .push_opcode(OP_DUP)
        .push_opcode(OP_DUP)
        .push_opcode(OP_DUP)
        .push_opcode(OP_TOALTSTACK) 
        .push_opcode(OP_TOALTSTACK) 
        .push_opcode(OP_ROT) 
        .push_opcode(OP_CAT)
        .push_opcode(OP_CAT)
        .push_opcode(OP_CAT)
        .push_opcode(OP_CAT) 
        .push_opcode(OP_SHA256) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_SWAP)
        .push_opcode(OP_CAT) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_ROT)
        .push_opcode(OP_SWAP) 
        .push_opcode(OP_DUP) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_CAT)
        .push_opcode(OP_ROT) 
        .push_opcode(OP_EQUALVERIFY) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_CAT)
        .push_opcode(OP_SWAP) 
        .push_opcode(OP_CHECKSIG);
    let script = builder.into_script();
    // println!("script: {:?}", script);
    script
}

pub(crate) fn htlc_refund_script(refund_address:&Address, lock_time: &i64) -> ScriptBuf {
    let mut builder = Script::builder();
    // The witness program needs to have the signature components except the outputs, prevouts,
    // followed by the previous transaction version, inputs, and locktime
    // followed by vault SPK, the vault amount, and the target SPK
    // followed by the fee-paying txout
    // followed by the mangled signature
    // and finally the a normal signature that signs with vault pubkey
    builder = builder
        .push_int(*lock_time)
        .push_opcode(OP_CSV)
        .push_opcode(OP_DROP)
        .push_opcode(OP_CSV) // check relative timelock on withdrawal
        .push_opcode(OP_DROP) // drop the result
        .push_opcode(OP_TOALTSTACK)
    .push_opcode(OP_TOALTSTACK)
    .push_opcode(OP_TOALTSTACK) 
    .push_slice(PushBytesBuf::try_from(hex::decode("0083020000000000000002").expect("Invalid secret hash hex")).unwrap())
    .push_opcode(OP_SWAP)
    .push_opcode(OP_CAT)
    .push_opcode(OP_SWAP)
    .push_opcode(OP_DUP) 
    .push_opcode(OP_TOALTSTACK)
    .push_opcode(OP_CAT)
    .push_opcode(OP_SWAP)
    .push_opcode(OP_CAT)
    .push_opcode(OP_FROMALTSTACK)
    // Push [length, script_pubkey_bytes...]
        .push_slice({
            let script_pubkey = refund_address.script_pubkey(); // Store Script
            let script_bytes = script_pubkey.as_bytes(); // Borrow
            let length = script_bytes.len() as u8;
            if script_bytes.len() > 255 {
                panic!("ScriptPubKey too long: {} bytes", script_bytes.len());
            }
            let mut bytes_with_length = vec![length];
            bytes_with_length.extend_from_slice(script_bytes);
            PushBytesBuf::try_from(bytes_with_length).expect("Invalid scriptPubKey bytes")
        })
    .push_opcode(OP_CAT)
    .push_opcode(OP_SHA256)
    .push_opcode(OP_CAT)
    .push_opcode(OP_SWAP)
    .push_opcode(OP_CAT)
    .push_slice(*TAPSIGHASH_TAG) // push tag
        .push_opcode(OP_SHA256) // hash tag
        .push_opcode(OP_DUP) // dup hash
        .push_opcode(OP_ROT) // move the sighash to the top of the stack
        .push_opcode(OP_CAT)
        .push_opcode(OP_CAT)
        .push_opcode(OP_SHA256) 
        .push_slice(*BIP0340_CHALLENGE_TAG) 
        .push_opcode(OP_SHA256)
        .push_opcode(OP_DUP)
        .push_opcode(OP_ROT) // bring challenge to the top of the stack
        .push_slice(*G_X) // G is used for the pubkey and K
        .push_opcode(OP_DUP)
        .push_opcode(OP_DUP)
        .push_opcode(OP_DUP)
        .push_opcode(OP_TOALTSTACK) 
        .push_opcode(OP_TOALTSTACK) 
        .push_opcode(OP_ROT) 
        .push_opcode(OP_CAT)
        .push_opcode(OP_CAT)
        .push_opcode(OP_CAT)
        .push_opcode(OP_CAT) 
        .push_opcode(OP_SHA256) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_SWAP)
        .push_opcode(OP_CAT) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_ROT)
        .push_opcode(OP_SWAP) 
        .push_opcode(OP_DUP) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_CAT)
        .push_opcode(OP_ROT) 
        .push_opcode(OP_EQUALVERIFY) 
        .push_opcode(OP_FROMALTSTACK) 
        .push_opcode(OP_CAT)
        .push_opcode(OP_SWAP) 
        .push_opcode(OP_CHECKSIG);
    let script = builder.into_script();
    println!("script: {:?}", script);
    script
}

pub(crate) fn htlc_instant_refund(x_only_pubkey: XOnlyPublicKey) {
   
}

