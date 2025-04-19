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
curl http://localhost:3000/btc/balance/1DrK44np3gMKuvcGeFVv9Jk67zodP52eMu
curl http://localhost:3000/btc/last-block-delta
```

---

## 🧠 Wallet Management & Persistence

> 📍 _Your Bitcoin Core node stores all blockchain data and wallet information on an **external SSD** mounted at `/mnt/bitcoin-data`._

This path **does not live inside your repository or VM image** — it is critical to remember this location when moving setups or rebuilding environments.

---

### 📁 Make sure `bitcoin.conf` exists

To load wallets automatically at boot, Bitcoin Core requires a properly configured `bitcoin.conf` file inside the data directory.

Use this command to edit it (you're a Vim person, after all):

```bash
vim /mnt/bitcoin-data/bitcoin.conf
```

Add the following:

```ini
# 📄 bitcoin.conf

txindex=1
rpcuser=bitcoin
rpcpassword=bitcoin123
rpcbind=0.0.0.0
rpcallowip=0.0.0.0/0
printtoconsole=1

# 🔑 Load the default wallet on startup
wallet=default
```

✅ This ensures the `default` wallet is created and loaded every time the node starts — **no more race conditions or manual execs needed**.

> ✨ You only need to do this once, but remember: this file lives in the **SSD**, not your project repo or the VM.

---

### 💼 Common Wallet Operations

#### 📥 Import a watch-only address

```bash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 importaddress "bc1q..." "watchaddr" false
```

#### 🔐 Import a private key

```bash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 importprivkey "cTp..." "mykey" false
```

#### 📊 View current wallet info

```bash
docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 getwalletinfo
```

---

### 📡 Test JSON-RPC Directly

You can ping the node via raw JSON-RPC to verify connectivity:

```bash
curl --user bitcoin:bitcoin123 \
  --data-binary '{"jsonrpc":"1.0","id":"test","method":"getblockchaininfo","params":[]}' \
  -H 'content-type:text/plain;' \
  http://localhost:8332/
```

---

## ✅ Status Check & Service Control

```bash
# 🔍 Check which services are running
docker ps

# 🚀 Start ONLY the Bitcoin service (no build needed — uses official image)
docker compose up -d bitcoin

# 🦀 Start the Rust API (Explorador Sigiloso)
# This command will recompile if you changed any Rust code
docker compose up -d --build explorador_api

# ⚡ Start both (builds the API, but bitcoin will just start as-is)
docker compose up -d --build

# 🧘 View logs for live debugging
docker logs -f bitcoin-mainnet
docker logs -f explorador-api
```


---

### 🧠 Developer Notes (2025-04-19, Saturday Sync Magic)

- The Bitcoin node uses a **mounted SSD at `/mnt/bitcoin-data`**. All chain data and wallets live there.
- The wallet auto-load is handled entirely via `bitcoin.conf` — no need for scripts or shell commands.
- The `explorador_api` service runs a Rust Axum backend and can panic if the wallet is not loaded. This is now fixed by ensuring `wallet=default` is present in config.
- You can always verify wallet loading with:
  ```bash
  docker exec -it bitcoin-mainnet bitcoin-cli -rpcuser=bitcoin -rpcpassword=bitcoin123 listwallets
  ```

---

### 🔮 Future Self: Expand Fearlessly

Your node stack is solid. It's modular. It's persistent. You're ready to build:

- ✅ `/btc/address/{address}/balance`
- ✅ `/btc/block/{height}`
- 🟡 `/eth/tx/{tx_hash}`
- 🟢 `/near/validator/{account_id}/rewards`

The forest is synced.  
The squirrel is alert.  
Your only job now... is to **create more endpoints** 🧠🐿️🚀



---

---

## 💾 External SSD Management (UTM + Mac)

If you're running the Bitcoin node with data stored on an external SSD, follow these steps **before physically disconnecting** the drive:

### ✅ Safely unmount the SSD

1. **Inside the UTM Linux VM**, run:

   ```bash
   sudo umount /mnt/bitcoin-data
   ```

   > This ensures the Linux system flushes all pending writes and releases the disk properly.

2. **On your Mac**, eject the SSD via Finder or run:

   ```bash
   diskutil unmount /Volumes/YOUR_DISK_NAME
   ```

   > Never unplug the SSD without unmounting first — especially during a blockchain sync. It can lead to data corruption and loss of progress.

### 🧠 Notes

- You can confirm unmount status in the VM using:

  ```bash
  lsblk
  ```

- Only unplug the disk once the mountpoint is gone.
- Your sync progress is valuable (especially after 60%!) — treat the SSD like sacred validator bark 🌳

---

---

## 🛑 Shutting Down the UTM VM Safely

Before unplugging the SSD or closing your laptop, always make sure to gracefully shut down the virtual machine to avoid data corruption — especially important when syncing blockchain data.

### ✅ Preferred Shutdown (from inside the VM)

Run this from the terminal:

```bash
sudo shutdown now
```

> This ensures all services are stopped, filesystems unmounted, and all caches flushed properly.

After this command, the VM will power off cleanly, and you can safely close UTM or disconnect your SSD.

---

### 🖥️ Alternative: Shutdown from UTM Interface

You can also shut down using the graphical interface:

1. Click the **Power** icon in the top-right corner of the UTM window.
2. Select **Request Power Down**.
   - This sends a safe ACPI shutdown signal to the guest OS (just like pressing a power button on a physical machine).
3. ✅ Wait until the VM window fully closes.

⚠️ **Never use "Force Shut Down" or "Kill VM"** unless absolutely necessary — it’s like yanking the power cable.

---

### 🧘 Sequence Before Unplugging SSD

1. Stop your Bitcoin node (or `docker compose down`).
2. Unmount the SSD from inside the VM:

   ```bash
   sudo umount /mnt/bitcoin-data
   ```

3. (Optional but recommended) Power down the disk:

   ```bash
   sudo udisksctl power-off -b /dev/sda
   ```

4. Shut down the VM:

   ```bash
   sudo shutdown now
   ```

5. Eject the SSD from your Mac, or just unplug it.

---

🧠 Pro Tip:
The fewer forced actions you take (pulling cables, killing VMs), the more peace your node will feel in the forest 🌲💾


### 👋 Final Notes

- This setup is for **mainnet**. If you're testing, replace `bitcoin-mainnet` with `bitcoin-testnet` and add `-testnet` to the command flags.
- The VM is your current host — you can later migrate this setup to a physical server with minimal changes.
- 1TB SSD is required for Bitcoin mainnet. Plan ahead for ETH if needed.

---

Made with ☕, 🧠, and a squirrel’s stubbornness.
