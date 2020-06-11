#[cfg(test)]
extern crate ethabi;
extern crate ethereum_types;
extern crate evm;
extern crate parity_bytes as bytes;
extern crate vm;

mod pure_ext;

use ethereum_types::{Address, H256, U256};
use evm::factory::Factory as EvmFactory;
use pure_ext::PureExt;
use std::sync::Arc;

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

pub type Result<T> = vm::Result<T>;

/// Returns the result of calling `data` on the given `code.
///
/// `code` takes the deployed code *not* initcode.
/// `data` is the normal transaction for calling a function, i.e.,
///        the sighash followed by abi-encoded arguments.
///
/// Assumes the function being called is a pure function.
pub fn exec(code: Vec<u8>, data: Vec<u8>) -> vm::Result<Vec<u8>> {
    let params = pure_action_params(code, data);
    let schedule = evm::Schedule::new_constantinople();

    let evm = {
        let depth = 0;
        let factory = EvmFactory::new(Default::default());
        factory.create(params.clone(), &schedule, depth)
    };

    let mut ext = PureExt::new(&schedule);

    let result = evm.exec(&mut ext);

    match result {
        Ok(r) => match r? {
            vm::GasLeft::NeedsReturn {
                gas_left: _,
                data,
                apply_state: _,
            } => Ok(data.to_vec()),
            _ => return Err(vm::Error::Internal("Invalid execution".to_string())),
        },
        Err(_) => return Err(vm::Error::Internal("Invalid execution".to_string())),
    }
}

fn pure_action_params(code: Vec<u8>, data: Vec<u8>) -> vm::ActionParams {
    let code_address = Address::zero();
    let code_hash: Option<H256> = None;
    let address = Address::zero();
    let sender = Address::zero();
    let origin = Address::zero();
    let gas: U256 = 50_000_000.into();
    let gas_price = U256::zero();
    let value = vm::ActionValue::Transfer(0.into());
    let code = Some(Arc::new(code));
    let data = Some(data);
    let call_type = vm::CallType::Call;
    let params_type = vm::ParamsType::Separate;
    let code_version = U256::zero();

    vm::ActionParams {
        code_address,
        code_hash,
        address,
        sender,
        origin,
        gas,
        gas_price,
        value,
        code,
        code_version,
        data,
        call_type,
        params_type,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_0_3() {
        let result = call_add_three(ethabi::Uint::from(0));
        assert_eq!(ethabi::Uint::from(3), result);
    }

    #[test]
    fn add_4_3() {
        let result = call_add_three(ethabi::Uint::from(4));
        assert_eq!(ethabi::Uint::from(7), result);
    }

    /// Executes a call to the following function.
    /// pragma solidity ^0.5.0;
    /// contract AddThree {
    ///   function addThree(uint256 a) public pure returns (uint256) {
    ///     return a + 3;
    ///   }
    /// }
    fn call_add_three(input: ethabi::Uint) -> ethabi::Uint {
        let code
            = hex!("608060405260043610603f576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806308316796146044575b600080fd5b348015604f57600080fd5b50607960048036036020811015606457600080fd5b8101908080359060200190929190505050608f565b6040518082815260200191505060405180910390f35b600060038201905091905056fea165627a7a723058200e912ad05dca5252a91d1ce28dda0451a49092178c344ac1a40ccf9c9d5d46150029").to_vec();

        let method = ethabi::Function {
            name: "addThree".to_owned(),
            inputs: vec![ethabi::Param {
                name: "a".to_owned(),
                kind: ethabi::ParamType::Uint(256),
            }],
            outputs: vec![ethabi::Param {
                name: "".to_owned(),
                kind: ethabi::ParamType::Uint(256),
            }],
            constant: true,
        };
        // ABI encode with input of 4.
        let data = method.encode_input(&[ethabi::Token::Uint(input)]).unwrap();
        let result = exec(code, data).unwrap();

        assert_eq!(result.len(), 32);
        ethabi::Uint::from(&result[..32])
    }
}
