extern crate crypto;
#[cfg(test)]
extern crate ethabi;
extern crate ethereum_types;
extern crate evm;
extern crate parity_bytes as bytes;
extern crate primitive_types;
extern crate secp256k1;
extern crate tiny_keccak;
extern crate vm;

mod pure_ext;

use ethereum_types::Address;
use evm::factory::Factory as EvmFactory;
use primitive_types::{H256, U256};
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
        let depth = 1;
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
    let action_type = vm::ActionType::Call;
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
        action_type,
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

    #[test]
    fn call_sha_precompile() {
        call_simple_hash();
    }

    #[test]
    fn call_recover_precompile() {
        call_simple_recover();
    }

    // Executes a call to the following function:
    // pragma solidity ^0.6.4;
    // contract SimpleHash {
    //     function hash(bytes memory toHash) public pure returns (bytes32) {
    //         return sha256(toHash);
    //     }
    // }
    fn call_simple_hash() {
        let deployed_bytecode = hex!("608060405234801561001057600080fd5b506004361061002b5760003560e01c8063aa1e84de14610030575b600080fd5b6100e96004803603602081101561004657600080fd5b810190808035906020019064010000000081111561006357600080fd5b82018360208201111561007557600080fd5b8035906020019184600183028401116401000000008311171561009757600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f8201169050808301925050505050505091929192905050506100ff565b6040518082815260200191505060405180910390f35b60006002826040518082805190602001908083835b602083106101375780518252602082019150602081019050602083039250610114565b6001836020036101000a038019825116818451168082178552505050505050905001915050602060405180830381855afa158015610179573d6000803e3d6000fd5b5050506040513d602081101561018e57600080fd5b8101908080519060200190929190505050905091905056fea2646970667358221220419ad886521e3b4d237dee58bc2a0ce4e93dd96342ed34753ec7162b1c4da8b264736f6c63430006040033").to_vec();

        // toHash = "0xabcd"
        let method_data = hex!("aa1e84de00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002abcd000000000000000000000000000000000000000000000000000000000000").to_vec();

        let result = exec(deployed_bytecode, method_data).unwrap();

        assert_eq!(result.len(), 32)
    }

    // Executes a call to the following function
    // pragma solidity ^0.6.4;
    // import "@openzeppelin/contracts/cryptography/ECDSA.sol";
    // contract SimpleRecover {
    //     function recover(bytes memory signature, bytes32 digest)
    //         public
    //         pure
    //         returns (address)
    //     {
    //         return ECDSA.recover(digest, signature);
    //     }
    // }
    fn call_simple_recover() {
        let deployed_bytecode = hex!("608060405234801561001057600080fd5b506004361061002b5760003560e01c80631aad75c514610030575b600080fd5b6100f36004803603604081101561004657600080fd5b810190808035906020019064010000000081111561006357600080fd5b82018360208201111561007557600080fd5b8035906020019184600183028401116401000000008311171561009757600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f82011690508083019250505050505050919291929080359060200190929190505050610135565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b60006101418284610149565b905092915050565b600060418251146101c2576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601f8152602001807f45434453413a20696e76616c6964207369676e6174757265206c656e6774680081525060200191505060405180910390fd5b60008060006020850151925060408501519150606085015160001a90507f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08260001c111561025b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260228152602001806103e56022913960400191505060405180910390fd5b601b8160ff16141580156102735750601c8160ff1614155b156102c9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260228152602001806104076022913960400191505060405180910390fd5b600060018783868660405160008152602001604052604051808581526020018460ff1660ff1681526020018381526020018281526020019450505050506020604051602081039080840390855afa158015610328573d6000803e3d6000fd5b505050602060405103519050600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614156103d7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260188152602001807f45434453413a20696e76616c6964207369676e6174757265000000000000000081525060200191505060405180910390fd5b809450505050509291505056fe45434453413a20696e76616c6964207369676e6174757265202773272076616c756545434453413a20696e76616c6964207369676e6174757265202776272076616c7565a2646970667358221220314ce2b06e8790839b6ab6605527f49fd44b16f06f0f2e670112bbb38d8b89ef64736f6c63430006040033").to_vec();

        let method_data = hex!("1aad75c500000000000000000000000000000000000000000000000000000000000000407378ae21851b80acc17e0110d13d265c43a3a9d91f7405c8329fae4def7a88090000000000000000000000000000000000000000000000000000000000000041f7ac299d8c3e49f0d51cbfd731f323b96ac86f90f02a2c213462108ba5cb0ef15b4b0182d5e1f2485195679ea071363101051e7b856639eb7802466b39a72f011c00000000000000000000000000000000000000000000000000000000000000").to_vec();

        let result = exec(deployed_bytecode, method_data).unwrap();

        assert_eq!(result.len(), 32)
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
