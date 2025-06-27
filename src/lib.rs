use std::fmt::{Debug, format};

use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // TODO: Decode hex string into Vec<u8>, return error string on failure
    let dec = hex::decode(hex_str).expect("failed to decode hex");
    Ok(dec)
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // TODO: Reverse the byte order of input slice and return as Vec<u8>
    let mut copy = bytes.to_vec();
    copy.reverse();
    copy
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // TODO: Implement conversion of bytes slice to hex string
    hex::encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // TODO: Implement conversion of hex string to bytes vector
    let res = decode_hex(hex).unwrap();
    Ok(res)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // TODO: Parse input string to u64, return error string if invalid
    let sats = input.parse();
    if sats.is_err() {
        return Err(String::from("Invalid satoshi amount"));
    }
    Ok(sats.unwrap())
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    // TODO: Match script pattern and return corresponding ScriptType
    match script {
        &[0x76, 0xa9, 0x14] => ScriptType::P2PKH,
        &[0x00, 0x14, 0xff] => ScriptType::P2WPKH,
        _ => ScriptType::Unknown,
    }
}

// TODO: complete Outpoint tuple struct
pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
    *balance = *balance - fee;
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
    format!("txid: {}", txid)
}

// TODO: Add necessary derive traits
#[derive(Debug, PartialEq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(String::from(format!("Invalid opcode: 0x{:02x}", &byte))),
        }
    }
}

// TODO: Add necessary derive traits
#[derive(Debug, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

impl UTXO {
    pub fn clone(&self) -> UTXO {
        let clone = UTXO {
            txid: self.txid.clone(),
            vout: self.vout,
            value: self.value,
        };

        clone
    }
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // TODO: Implement UTXO consumption logic (if any)
    utxo
}
