use bytes::{Bytes, BytesRef};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use ethereum_types::{Address, H256, U256};
// use parity_crypto::digest;
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
            // let mut o = [255u8; 32];
            // EcRecover
            //     .execute(data, &mut BytesRef::Fixed(&mut o))
            //     .expect("ecrecover failed");
            // return Ok(MessageCallResult::Success(
            //     gas.clone(),
            //     ReturnData::new(Vec::from(&o[..]), 0, 32),
            // ));
        } else if receive_address == &Address::from_low_u64_be(2) {
            let mut o = [255u8; 32];
            let mut digest = Sha256::new();
            digest.input(&data);
            digest.result(&mut o);
            return Ok(MessageCallResult::Success(
                gas.clone(),
                ReturnData::new(Vec::from(&o[..]), 0, 32),
            ));
        }
        unimplemented!();
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
