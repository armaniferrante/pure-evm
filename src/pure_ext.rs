use bytes::Bytes;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use ethereum_types::{Address, H256, U256};
use secp256k1::{recover, Message, RecoveryId, Signature};
use std::cmp::min;
use std::sync::Arc;
use vm::{
    ActionType, ContractCreateResult, CreateContractAddress, EnvInfo, Ext, MessageCallResult,
    Result, ReturnData, Schedule, TrapKind,
};

/// Externalities for an EVM supporting pure functions only.
pub struct PureExt<'a> {
    schedule: &'a Schedule,
}

impl<'a> PureExt<'a> {
    pub fn new(schedule: &'a Schedule) -> Self {
        PureExt { schedule }
    }
}

impl<'a> Ext for PureExt<'a> {
    fn initial_storage_at(&self, _key: &H256) -> Result<H256> {
        unimplemented!();
    }

    fn storage_at(&self, _key: &H256) -> Result<H256> {
        unimplemented!();
    }

    fn set_storage(&mut self, _key: H256, _value: H256) -> Result<()> {
        unimplemented!();
    }

    fn exists(&self, _address: &Address) -> Result<bool> {
        unimplemented!();
    }

    fn exists_and_not_null(&self, _address: &Address) -> Result<bool> {
        unimplemented!();
    }

    fn origin_balance(&self) -> Result<U256> {
        unimplemented!();
    }

    fn balance(&self, _address: &Address) -> Result<U256> {
        unimplemented!();
    }

    fn blockhash(&mut self, _number: &U256) -> H256 {
        unimplemented!();
    }

    fn create(
        &mut self,
        _gas: &U256,
        _value: &U256,
        _code: &[u8],
        _parent_version: &U256,
        _address: CreateContractAddress,
        _trap: bool,
    ) -> ::std::result::Result<ContractCreateResult, TrapKind> {
        unimplemented!();
    }

    fn call(
        &mut self,
        gas: &U256,
        _sender_address: &Address,
        receive_address: &Address,
        _value: Option<U256>,
        data: &[u8],
        _code_address: &Address,
        _action_type: ActionType,
        _trap: bool,
    ) -> ::std::result::Result<MessageCallResult, TrapKind> {
        if receive_address == &Address::from_low_u64_be(1) {
            match ecrecover(&data) {
                Ok(out) => {
                    let len = out.len();
                    Ok(MessageCallResult::Success(
                        gas.clone(),
                        ReturnData::new(out, 0, len),
                    ))
                }
                Err(_) => {
                    // Figure out a way to report the error; outside
                    // WASM we could log it, but here it's not clear
                    // what to do
                    Ok(MessageCallResult::Failed)
                }
            }
        } else if receive_address == &Address::from_low_u64_be(2) {
            let out = sha256(&data);
            let len = out.len();
            Ok(MessageCallResult::Success(
                gas.clone(),
                ReturnData::new(out, 0, len),
            ))
        } else {
            unimplemented!();
        }
    }

    fn extcode(&self, _address: &Address) -> Result<Option<Arc<Bytes>>> {
        unimplemented!();
    }

    fn extcodehash(&self, _address: &Address) -> Result<Option<H256>> {
        unimplemented!();
    }

    fn extcodesize(&self, _address: &Address) -> Result<Option<usize>> {
        unimplemented!();
    }

    fn log(&mut self, _topics: Vec<H256>, _data: &[u8]) -> Result<()> {
        unimplemented!();
    }

    fn ret(self, _gas: &U256, _data: &ReturnData, _apply_state: bool) -> Result<U256> {
        // todo
        unimplemented!();
    }

    fn suicide(&mut self, _refund_address: &Address) -> Result<()> {
        unimplemented!();
    }

    fn schedule(&self) -> &Schedule {
        self.schedule
    }

    fn env_info(&self) -> &EnvInfo {
        unimplemented!();
    }

    fn chain_id(&self) -> u64 {
        unimplemented!();
    }

    fn depth(&self) -> usize {
        // Assume the contract tested does not make calls to other contracts,
        // hence the execution depth should always be 0.
        0
    }

    fn add_sstore_refund(&mut self, _value: usize) {
        unimplemented!();
    }

    fn sub_sstore_refund(&mut self, _value: usize) {
        unimplemented!();
    }

    fn trace_next_instruction(&mut self, _pc: usize, _instruction: u8, _current_gas: U256) -> bool {
        false
    }

    fn trace_prepare_execute(
        &mut self,
        _pc: usize,
        _instruction: u8,
        _gas_cost: U256,
        _mem_written: Option<(usize, usize)>,
        _store_written: Option<(U256, U256)>,
    ) {
        unimplemented!();
    }

    fn trace_executed(&mut self, _gas_used: U256, _stack_push: &[U256], _mem: &[u8]) {
        unimplemented!();
    }

    fn is_static(&self) -> bool {
        unimplemented!();
    }
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut out = [255u8; 32];
    let mut digest = Sha256::new();
    digest.input(data);
    digest.result(&mut out);
    out[..].into()
}

fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hash = [0u8; 32];
    tiny_keccak::Keccak::keccak256(&input, &mut hash);
    hash
}

const HASH_OFFSET: usize = 0;
const HASH_LENGTH: usize = 32;
const REC_ID_OFFSET: usize = HASH_LENGTH;
const REC_ID_LENGTH: usize = 32;
const COORD_OFFSET: usize = REC_ID_OFFSET + REC_ID_LENGTH;
const COORD_LENGTH: usize = 32;
const SIG_OFFSET: usize = COORD_OFFSET + COORD_LENGTH;
const SIG_LENGTH: usize = 32;

/// Copied from https://github.com/ewasm/ewasm-precompiles/blob/master/ecrecover/src/lib.rs.
fn ecrecover(input: &[u8]) -> std::result::Result<Vec<u8>, secp256k1::Error> {
    let hash_start = min(HASH_OFFSET, input.len());
    let hash_end = min(HASH_OFFSET + HASH_LENGTH, input.len());
    let mut h = [0u8; HASH_LENGTH];
    for (i, val) in (&input[hash_start..hash_end]).iter().enumerate() {
        h[i] = *val;
    }

    /* Recovery id is the last big-endian byte. */
    let v = if input.len() > REC_ID_OFFSET + REC_ID_LENGTH - 1 {
        (input[REC_ID_OFFSET + REC_ID_LENGTH - 1] as i8 - 27) as u8
    } else {
        (256 - 27) as u8 /* Assume the padding would yield 0 */
    };
    if v != 0 && v != 1 {
        return Ok(vec![0u8; 0]);
    }

    let sig_start = min(COORD_OFFSET, input.len());
    let sig_end = min(SIG_OFFSET + SIG_LENGTH, input.len());
    let mut s = [0u8; 64];
    for (i, val) in (&input[sig_start..sig_end]).iter().enumerate() {
        s[i] = *val;
    }

    let message = Message::parse(&h);
    let rec_id = RecoveryId::parse(v)?;
    let sig = Signature::parse(&s);

    let key = recover(&message, &sig, &rec_id)?;
    let ret = key.serialize();
    let ret = keccak(&ret[1..65]);
    let mut output = vec![0u8; 12];
    output.extend_from_slice(&ret[12..32]);
    Ok(output)
}
