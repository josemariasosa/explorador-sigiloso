🧠 Desired Flow: _safe + _api + Ethereum
You want something like this:

🧱 Scenario: Deploying a Contract (e.g. Foundry-built Solidity contract)
_api prepares the unsigned transaction: nonce, gas, calldata, value, chainId.

_api sends the transaction data (not signed!) to _safe.

_safe signs it and returns the signed raw tx.

_api sends the signed tx to your reth Ethereum node (via JSON-RPC).

Done ✅ — contract is deployed, account paid for gas, key never left _safe.


🧱 What a TX Looks Like (ETH)
An Ethereum transaction needs:

json
Copy
Edit
{
  "nonce": 1,
  "gas": 21000,
  "to": null, // for contract deployment
  "value": 0,
  "data": "0x..." // bytecode + constructor args
}