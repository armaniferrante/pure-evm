// Referencing the built package. Replace with require('pure-evm').
const pure_evm = require('../../pkg');
const assert = require('assert');

const output = pure_evm.exec(bytecode(), data());

const expected = Buffer.from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7]);

assert.equal(expected.toString('hex'), Buffer.from(output).toString('hex'));

/**
 * Deployed bytecode *not* the initcode for the following contract.
 *
 * pragma solidity ^0.5.0;
 * contract AddThree {
 *   function addThree(uint256 a) public pure returns (uint256) {
 *     return a + 3;
 *     }
 * }
 */
function bytecode() {
  let bytecode = '608060405260043610603f576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806308316796146044575b600080fd5b348015604f57600080fd5b50607960048036036020811015606457600080fd5b8101908080359060200190929190505050608f565b6040518082815260200191505060405180910390f35b600060038201905091905056fea165627a7a723058200e912ad05dca5252a91d1ce28dda0451a49092178c344ac1a40ccf9c9d5d46150029';
  return Buffer.from(bytecode, 'hex');
}

/**
 * Data field for transaction to `addThree` function with a = 4.
 */
function data() {
  let data = '083167960000000000000000000000000000000000000000000000000000000000000004';
  return Buffer.from(data, 'hex');
}
