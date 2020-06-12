/**
 * Tests against the contract here:
 * https://github.com/connext/indra/blob/staging/modules/contracts/src.sol/apps/SimpleLinkedTransferApp.sol
 */

const pure_evm = require("../../pkg-node");
const assert = require("assert");
const types = require("@connext/types");
const contracts = require("@connext/contracts");
const connextUtils = require("@connext/utils");
const ethers = require("ethers");
const {
  getTestReceiptToSign,
  signReceiptMessage,
  getTestVerifyingContract,
} = require("@connext/utils");
const {
  SimpleSignedTransferAppStateEncoding,
  SimpleSignedTransferAppActionEncoding,
} = require("@connext/types");

const { Wallet, constants, utils } = ethers;
const { soliditySha256, defaultAbiCoder, Interface } = utils;
const { HashZero, Zero, One } = constants;

const { SimpleLinkedTransferApp, SimpleSignedTransferApp } = contracts;
const { getRandomBytes32 } = connextUtils;
const {
  SimpleLinkedTransferAppStateEncoding,
  SimpleLinkedTransferAppActionEncoding,
} = types;

const sender = Wallet.createRandom();
const receiver = Wallet.createRandom();
const randomBytes = getRandomBytes32();

(async () => {
  // Execute on the raw bytes.
  const shaBytecode = Uint8Array.from(
    Buffer.from(
      SimpleLinkedTransferApp.deployedBytecode.replace("0x", ""),
      "hex"
    )
  );
  console.log("Executing sha precompile call on evm");
  const shaOutput = pure_evm.exec(shaBytecode, linkedData());
  // First abi decode bytes, since that's the return value of the function.
  const shaBytes = defaultAbiCoder.decode(["bytes"], shaOutput)[0];
  // Now decode the state data, since that's what the bytes represent.
  const shaDecoded = defaultAbiCoder.decode(
    [SimpleLinkedTransferAppStateEncoding],
    shaBytes
  )[0];
  console.log("Decoded result", shaDecoded);
  // Assert the state was properly updated.
  const { state: shaState, action: shaAction } = linkedState();
  assert.equal(
    decodedResult.coinTransfers + "",
    [
      [sender.address, Zero],
      [receiver.address, One],
    ] + ""
  );
  assert.equal(decodedResult.finalized + "", true + "");
  assert.equal(decodedResult.linkedHash + "", shaState.linkedHash + "");
  assert.equal(decodedResult.preImage + "", shaAction.preImage + "");

  console.log("Precompiles sha success. Beginning ecrecover test.");

  // execute for ecrecover precompile
  console.log(
    `ecrecover deployed:`,
    SimpleSignedTransferApp.deployedBytecode.replace("0x", "")
  );
  const bytecode = Uint8Array.from(
    Buffer.from(
      SimpleSignedTransferApp.deployedBytecode.replace("0x", ""),
      "hex"
    )
  );
  console.log("Executing ecrecover precompile call on evm");
  const output = pure_evm.exec(bytecode, await signedData());
  // First abi decode bytes, since that's the return value of the function.
  const bytes = defaultAbiCoder.decode(["bytes"], output)[0];
  // Now decode the state data, since that's what the bytes represent.
  const decodedResult = defaultAbiCoder.decode(
    [SimpleSignedTransferAppStateEncoding],
    bytes
  )[0];
  console.log("Decoded result", decodedResult);
  // Assert the state was properly updated.
  const { state } = await signedState();
  assert.equal(
    decodedResult.coinTransfers + "",
    [
      [sender.address, Zero],
      [receiver.address, One],
    ] + ""
  );
  assert.equal(decodedResult.finalized + "", true + "");
  assert.equal(decodedResult.chainId + "", state.chainId + "");
  assert.equal(decodedResult.signerAddress + "", state.signerAddress + "");
  assert.equal(
    decodedResult.verifyingContact + "",
    state.verifyingContact + ""
  );
  assert.equal(decodedResult.requestCID + "", state.requestCID + "");
  assert.equal(
    decodedResult.verifyingContact + "",
    state.verifyingContact + ""
  );
  assert.equal(
    decodedResult.subgraphDeploymentId + "",
    state.subgraphDeploymentId + ""
  );
  assert.equal(decodedResult.paymentId + "", state.paymentId + "");

  console.log("Precompiles ecrecover success.");
})();

/////// Helper functions
function linkedState() {
  const preImage = randomBytes;
  const state = {
    linkedHash: soliditySha256(["bytes32"], [preImage]),
    finalized: false,
    preImage: HashZero,
    coinTransfers: [
      {
        to: sender.address,
        amount: One,
      },
      {
        to: receiver.address,
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
  const data = iface.encodeFunctionData("applyAction", [
    encodedState,
    encodedAction,
  ]);
  return Uint8Array.from(Buffer.from(data.replace("0x", ""), "hex"));
}

async function signedState() {
  const receipt = getTestReceiptToSign();
  const state = {
    coinTransfers: [
      {
        to: sender.address,
        amount: One,
      },
      {
        to: receiver.address,
        amount: Zero,
      },
    ],
    signerAddress: receiver.address,
    chainId: 1337,
    verifyingContact: getTestVerifyingContract(),
    requestCID: receipt.requestCID,
    subgraphDeploymentId: receipt.subgraphDeploymentID,
    paymentId: randomBytes,
    finalized: false,
  };
  const signature = await signReceiptMessage(
    receipt,
    state.chainId,
    state.verifyingContact,
    receiver.privateKey
  );
  const action = {
    resonseCID: receipt.responseCID,
    signature,
  };
  return { state, action };
}

async function signedData() {
  const { state, action } = await signedState();

  const encodedState = defaultAbiCoder.encode(
    [SimpleSignedTransferAppStateEncoding],
    [state]
  );
  const encodedAction = defaultAbiCoder.encode(
    [SimpleSignedTransferAppActionEncoding],
    [action]
  );
  const iface = new Interface(SimpleSignedTransferApp.abi);
  const data = iface.encodeFunctionData("applyAction", [
    encodedState,
    encodedAction,
  ]);
  console.log(`ecrecover fn data:`, data.replace("0x", ""));
  return Uint8Array.from(Buffer.from(data.replace("0x", ""), "hex"));
}
