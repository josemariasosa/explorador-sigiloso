# Nodo Sigiloso Infra

> Infrastructure setup for running a full Bitcoin node and the [`explorador-sigiloso`](https://github.com/josemariasosa/explorador-sigiloso) dashboard on a local or dedicated Linux server (physical or virtual).

---

## 💾 Format the External SSD

This assumes the external SSD has been connected to the VM or physical server. You'll format it with EXT4 so Bitcoin Core can write to it efficiently.

```bash
# Start a temporary Debian container with disk access
docker-compose up -d debian
docker exec -it debian-formatter bash

# Install necessary tools
apt update && apt install -y util-linux e2fsprogs procps lsof

# List block devices
lsblk

# Format the drive (⚠️ make sure it's the correct one)
mkfs.ext4 /dev/sda -L bitcoin_data
```

Then, on the host system:

```bash
# Mount the newly formatted SSD
sudo mkdir -p /mnt/bitcoin-data
sudo mount /dev/sda /mnt/bitcoin-data

df -h /mnt/bitcoin-data/
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda        916G   28K  870G   1% /mnt/bitcoin-data
```

---

## 🐳 Docker Compose

Your server should have Docker and Docker Compose installed. Then clone the repo:

```bash
git clone git@github.com:josemariasosa/explorador-sigiloso.git
cd explorador-sigiloso
```

Sample `docker-compose.yml` (Bitcoin node + API):

```yaml
services:
  bitcoin:
    image: bitcoin/bitcoin:latest
    container_name: bitcoin-mainnet
    restart: unless-stopped
    ports:
      - "8332:8332"  # RPC
      - "8333:8333"  # P2P
    volumes:
      - /mnt/bitcoin-data:/home/bitcoin/.bitcoin
    command:
      -printtoconsole
      -rpcallowip=0.0.0.0/0
      -rpcbind=0.0.0.0
      -rpcuser=bitcoin
      -rpcpassword=bitcoin123
      -txindex=1

  explorador_api:
    build:
      context: ./explorador_sigiloso_api
    container_name: explorador-api
    restart: unless-stopped
    ports:
      - "3000:3000"
    environment:
      BTC_RPC_URL: http://bitcoin:8332
      BTC_RPC_USER: bitcoin
      BTC_RPC_PASS: bitcoin123
    command: ["cargo", "run", "--release"]
    
```

---

## 🛠️ Load Wallets

```bash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 createwallet default
```

---

## 🧪 Test Bitcoin Node Status

```bash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 getblockchaininfo
```

---

## 🔍 Query Blockchain Data

### From Bitcoin CLI:

```bash
# Get latest block hash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 getbestblockhash

# Get block details (verbosity 2)
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 getblock <BLOCK_HASH> 2
```

### From Explorador API (Axum):

```bash
curl http://localhost:3000/btc/balance/1K8jWKBgWU2L1zvBbTn4G3vMyJx8Ra1J6G
```

---

## 🧠 Wallet Tips

### Import a watch-only address:

```bash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 importaddress "bc1q..." "watchaddr" false
```

### Import a private key:

```bash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 importprivkey "cTp..." "mykey" false
```

---

## 🔐 Check Wallet Info

```bash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 getwalletinfo
```

---

## 📡 Test JSON-RPC Directly

```bash
curl --user bitcoin:bitcoin123   --data-binary '{"jsonrpc":"1.0","id":"test","method":"getblockchaininfo","params":[]}'   -H 'content-type:text/plain;'   http://localhost:8332/
```

---

## ✅ Status Check

```bash
# Running containers
docker ps

✅ Run only the bitcoin service:
docker compose up -d bitcoin

# Live logs
docker logs -f bitcoin-mainnet
docker logs -f explorador-api
```

---

### 👋 Final Notes

- This setup is for **mainnet**. If you're testing, replace `bitcoin-mainnet` with `bitcoin-testnet` and add `-testnet` to the command flags.
- The VM is your current host — you can later migrate this setup to a physical server with minimal changes.
- 1TB SSD is required for Bitcoin mainnet. Plan ahead for ETH if needed.

---

Made with ☕, 🧠, and a squirrel’s stubbornness.
