/**
 * Tests against the contract here:
 * https://github.com/connext/indra/blob/staging/modules/contracts/src.sol/apps/SimpleLinkedTransferApp.sol
 */

const pure_evm = require("../../pkg");
const assert = require("assert");
const types = require("@connext/types");
const contracts = require("@connext/contracts");
const connextUtils = require("@connext/utils");
const ethers = require("ethers");

const { Wallet, constants, utils } = ethers;
const { soliditySha256, defaultAbiCoder, Interface } = utils;
const { HashZero, Zero, One } = constants;

const { SimpleLinkedTransferApp } = contracts;
const { getRandomBytes32 } = connextUtils;
const {
  SimpleLinkedTransferAppStateEncoding,
  SimpleLinkedTransferAppActionEncoding,
} = types;

const sender = Wallet.createRandom().address;
const receiver = Wallet.createRandom().address;

// Execute on the raw bytes.
const bytecode = Uint8Array.from(
  Buffer.from(SimpleLinkedTransferApp.deployedBytecode.replace("0x", ""), "hex")
);
console.log("Executing precompile call on evm");
const output = pure_evm.exec(bytecode, linkedData());
// First abi decode bytes, since that's the return value of the function.
const bytes = defaultAbiCoder.decode(["bytes"], output)[0];
// Now decode the state data, since that's what the bytes represent.
const decodedResult = defaultAbiCoder.decode(
  [SimpleLinkedTransferAppStateEncoding],
  bytes
)[0];
console.log("Decoded result", decodedResult);
// Assert the state was properly updated.
const { state, action } = linkedState();
assert.equal(
  decodedResult.coinTransfers + "",
  [
    [sender, Zero],
    [receiver, One],
  ] + ""
);
assert.equal(decodedResult.finalized + "", state.finalized + "");
assert.equal(decodedResult.linkedHash + "", state.linkedHash + "");
assert.equal(decodedResult.preImage + "", action.preImage + "");

console.log("Precompiles success.");

function linkedState() {
  const preImage = getRandomBytes32();
  const state = {
    linkedHash: soliditySha256(["bytes32"], [preImage]),
    finalized: false,
    preImage: HashZero,
    coinTransfers: [
      {
        to: sender,
        amount: One,
      },
      {
        to: receiver,
        amount: Zero,
      },
    ],
  };
  const action = { preImage };
  return { state, action };
}

function linkedData() {
  const { state, action } = linkedState();

  const encodedState = defaultAbiCoder.encode(
    [SimpleLinkedTransferAppStateEncoding],
    [state]
  );
  const encodedAction = defaultAbiCoder.encode(
    [SimpleLinkedTransferAppActionEncoding],
    [action]
  );
  const iface = new Interface(SimpleLinkedTransferApp.abi);
  const data = iface.functions.applyAction.encode([
    encodedState,
    encodedAction,
  ]);
  return Uint8Array.from(Buffer.from(data.replace("0x", ""), "hex"));
}
