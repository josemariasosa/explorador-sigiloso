# Nodo Sigiloso Infra

docs:

- https://hub.docker.com/r/bitcoin/bitcoin


docker ps
docker logs -f bitcoin-testnet

curl --user bitcoin:bitcoin123 --data-binary '{"jsonrpc":"1.0","id":"test","method":"getblockchaininfo","params":[]}' \
  -H 'content-type:text/plain;' http://localhost:18332/

{"result":{"chain":"test","blocks":0,"headers":0,"bestblockhash":"000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943","difficulty":1,"time":1296688602,"mediantime":1296688602,"verificationprogress":3.905231570194885e-09,"initialblockdownload":true,"chainwork":"0000000000000000000000000000000000000000000000000000000100010001","size_on_disk":293,"pruned":false,"warnings":[]},"error":null,"id":"test"}

Step 2: Load your Bitcoin Core wallet (if not done yet)
bash
Copy
Edit
```sh
docker ps
docker-compose up -d

docker exec -it bitcoin-testnet bitcoin-cli -testnet -rpcuser=bitcoin -rpcpassword=bitcoin123 createwallet default
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 createwallet default

# check the status of the node
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 getblockchaininfo
{
  "chain": "main",
  "blocks": 193868,
  "headers": 892878,
  "bestblockhash": "00000000000005478a33ec50952f514f323f80c0677c7055f8d570e46380f71c",
  "difficulty": 2190865.970102859,
  "time": 1344956751,
  "mediantime": 1344954130,
  "verificationprogress": 0.004956699604573066,
  "initialblockdownload": true,
  "chainwork": "0000000000000000000000000000000000000000000000173478f6c50d31e393",
  "size_on_disk": 2948976382,
  "pruned": false,
  "warnings": [
  ]
}

```

Query the blockchain to get the LAST BLOCK.

```sh
✅ 1. Get the latest block hash

docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 getbestblockhash

000000000000002f7dae984ed518b68bde8d9d5ef666eae58d1a6bc11136fed1

✅ 2. Fetch the block by hash

docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 getblock 000000000000002f7dae984ed518b68bde8d9d5ef666eae58d1a6bc11136fed1 2
```

curl http://localhost:3000/btc/balance/yourbtcaddresshere


# test the running node
curl --user bitcoin:bitcoin123 \
  --data-binary '{"jsonrpc":"1.0","id":"btcbalance","method":"getbalance","params":[]}' \
  -H 'content-type:text/plain;' http://localhost:8332/

```


Step 3: Import the address or private key
👉 Option A: Import just the address (for watching only)
bash
Copy
Edit
docker exec -it bitcoin-testnet bitcoin-cli -testnet -rpcuser=bitcoin -rpcpassword=bitcoin123 importaddress "tb1q...." "mywatchaddr" false
👉 Option B: Import the private key (you'll be able to spend/test from it)
bash
Copy
Edit
docker exec -it bitcoin-testnet bitcoin-cli -testnet -rpcuser=bitcoin -rpcpassword=bitcoin123 importprivkey "cTp...." "mykey" false
The "false" at the end skips rescan for speed. If you want to scan past UTXOs, set it to true.

Step 4: Verify it's working
bash
Copy
Edit
docker exec -it bitcoin-testnet bitcoin-cli -testnet -rpcuser=bitcoin -rpcpassword=bitcoin123 getwalletinfo
Then test your API again:

bash
Copy
Edit
curl http://localhost:3000/btc/balance/yourTestnetAddress