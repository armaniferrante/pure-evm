# Pure-Evm

A browser and node compatible EVM interpreter for pure function calls compiled to WebAssembly.

## Usage

`npm install pure-evm`

The interface `pure_evm.exec()` takes in a contract's `bytecode` along with a transaction's `data` field and returns a `Uint8Array` of the resultant transaction output. Note that the bytecode should be the bytecode stored onchain, i.e., *not* the initcode. `data` is simply the data field of the transaction to invoke the pure function, i.e., the method's sighash followed by abi encoded arguments.

### Browser

Because `pure-evm` is loaded as a WebAssembly module in browser, it must be loaded asynchronously.

For example,

```javascript
import('pure-evm').then((pure_evm) => {

  let output = pure_evm.exec(bytecode(), data())

  console.log(output);
});

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
  return Uint8Array.from(Buffer.from(bytecode, 'hex'));
}

/**
 * Data field for transaction to `addThree` function with a = 4.
 */
function data() {
  let data = '083167960000000000000000000000000000000000000000000000000000000000000004';
  return Uint8Array.from(Buffer.from(data, 'hex'));
}

```

## Node

In node environments, `pure-evm` can be used in a similar way.

```javascript
const pure_evm = require('pure-evm');
...
```

See the examples [here](https://github.com/armaniferrante/pure-evm/tree/master/wasm/examples).
