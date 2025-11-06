# FintraDex Parachain Deployment Guide

This guide provides comprehensive instructions for deploying the FintraDex parachain on Paseo Testnet and preparing for mainnet deployment.

---

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Development Deployment](#development-deployment)
- [Testnet Deployment (Paseo)](#testnet-deployment-paseo)
- [Production Deployment](#production-deployment)
- [Troubleshooting](#troubleshooting)
- [Security Best Practices](#security-best-practices)

---

## Prerequisites

This section covers the installation of Polkadot SDK dependencies required for building and deploying the FintraDex parachain. For detailed information, refer to the [official Polkadot SDK installation guide](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/#verifying-installation).

### System Requirements

- **Operating System**: Linux (Ubuntu 20.04+ recommended), macOS, or Windows with WSL2
- **CPU**: 4+ cores recommended
- **RAM**: 16GB+ recommended
- **Storage**: 100GB+ SSD for blockchain data
- **Network**: Stable internet connection with open ports

### macOS Installation

#### Before You Begin

**Install Homebrew:**
If you don't have Homebrew installed, install it first:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

**Support for Apple Silicon:**
If you're using an Apple Silicon Mac (M1/M2/M3), ensure you have Rosetta 2 installed if needed for compatibility.

#### Install Required Packages and Rust

1. **Open Terminal application**

2. **Update Homebrew:**
   ```bash
   brew update
   ```

3. **Install OpenSSL:**
   ```bash
   brew install openssl
   ```
   > Note: OpenSSL is required for cryptography operations including public/private key pair generation and transaction signature validation.

4. **Install Rust using rustup:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Follow the prompts to proceed with default installation.

5. **Update your shell environment:**
   ```bash
   source ~/.cargo/env
   ```

6. **Configure Rust toolchain:**
   ```bash
   rustup default stable
   rustup update
   rustup target add wasm32-unknown-unknown
   rustup component add rust-src
   ```

7. **Install CMake:**
   ```bash
   brew install cmake
   ```

8. **Verify installation:**
   ```bash
   rustup show
   ```
   You should see output showing your active toolchain with `wasm32-unknown-unknown` target installed.

### Linux Installation

#### Before You Begin

Check your Linux distribution documentation for package management. At minimum, you need:
- A linker or C-compatible compiler (e.g., `clang`)
- `curl`, `git`, `make`
- Cryptography package (`libssl-dev` or `openssl-devel`)

#### Install Required Packages and Rust

**For Ubuntu/Debian:**
```bash
sudo apt install --assume-yes git clang curl libssl-dev protobuf-compiler
```

**For Debian (with additional packages):**
```bash
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
```

**For Arch Linux:**
```bash
pacman -Syu --needed --noconfirm curl git clang make protobuf
```

**For Fedora:**
```bash
sudo dnf update
sudo dnf install clang curl git openssl-devel make protobuf-compiler
```

**For OpenSUSE:**
```bash
sudo zypper install clang curl git openssl-devel llvm-devel libudev-devel make protobuf
```

**Install Rust:**
1. **Download and install rustup:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Follow the prompts for default installation.

2. **Update your shell environment:**
   ```bash
   source $HOME/.cargo/env
   ```

3. **Verify Rust installation:**
   ```bash
   rustc --version
   ```

4. **Configure Rust toolchain:**
   ```bash
   rustup default stable
   rustup update
   rustup target add wasm32-unknown-unknown
   rustup component add rust-src
   ```

5. **Verify installation:**
   ```bash
   rustup show
   ```
   You should see `wasm32-unknown-unknown` in the installed targets.

### Windows (WSL) Installation

#### Before You Begin

**Windows Requirements:**
- **Windows 10**: Version 2004 (Build 19041) or later
- **Windows 11**: Any version
- **Windows Server**: 2019 or later

#### Set Up Windows Subsystem for Linux

1. **Check Windows version:**
   - Windows 10 version 2004+ or Windows 11: WSL is available by default
   - Older versions: See [WSL manual installation](https://learn.microsoft.com/en-us/windows/wsl/install-manual)

2. **Open PowerShell or Command Prompt as Administrator:**
   - Right-click **Start** menu → **Windows PowerShell** or **Command Prompt** → **Run as administrator**

3. **Install WSL:**
   ```powershell
   wsl --install
   ```
   This enables WSL 2, downloads the latest Linux kernel, and installs Ubuntu by default.

4. **View available Linux distributions (optional):**
   ```powershell
   wsl --list --online
   ```

5. **Restart your computer** to complete the installation.

#### Install Required Packages and Rust

1. **Open Ubuntu** from the Start menu

2. **Create a UNIX user account** (username and password)

3. **Update Ubuntu packages:**
   ```bash
   sudo apt update
   ```

4. **Install required packages:**
   ```bash
   sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
   ```

5. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Follow the prompts for default installation.

6. **Update your shell environment:**
   ```bash
   source ~/.cargo/env
   ```

7. **Verify Rust installation:**
   ```bash
   rustc --version
   ```

8. **Configure Rust toolchain:**
   ```bash
   rustup default stable
   rustup update
   rustup target add wasm32-unknown-unknown
   rustup component add rust-src
   ```

9. **Verify installation:**
   ```bash
   rustup show
   ```

### Verifying Installation

After completing the installation for your platform, verify your setup:

```bash
rustup show
```

Expected output should show:
- Active toolchain (e.g., `stable-x86_64-unknown-linux-gnu` or `stable-aarch64-apple-darwin`)
- Installed targets including `wasm32-unknown-unknown`

### Additional Required Tools

After installing the base dependencies, install the following tools:

```bash
# Install chain-spec-builder
cargo install --locked staging-chain-spec-builder@10.0.0

# Install polkadot-omni-node
cargo install --locked polkadot-omni-node@0.5.0

# Install subkey (for key generation)
cargo install --force subkey --git https://github.com/paritytech/polkadot-sdk --tag v1.0.0
```

### Build the Project

```bash
# Clone the repository
git clone https://github.com/fintradev/fintradex.git
cd fintradex

# Build release binary
cargo build --release

# Verify build
./target/release/fintradex-node --version
```

### Where to Go Next

- **Polkadot SDK Documentation**: [Install Polkadot SDK Dependencies](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/#verifying-installation)
- **Parachain Zero to Hero Tutorials**: Step-by-step guides for building, testing, and deploying custom pallets and runtimes

---

## Development Deployment

### Local Development Node

For local testing and development:

```bash
# Generate development chain spec
chain-spec-builder create -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/fintradex-runtime/fintradex_runtime.compact.compressed.wasm \
  named-preset development

# Start development node
polkadot-omni-node --chain ./chain_spec.json --dev
```

The node will start with:
- Pre-funded accounts
- Single collator
- Fast block times
- All features enabled

### Local Testnet with Zombienet

For multi-node testing:

```bash
# Ensure you have zombienet installed
cargo install --locked zombienet

# Run zombienet configuration
zombienet spawn zombienet.toml
```

---

## Testnet Deployment (Paseo)

### Step 1: Prepare Testnet Environment

#### 1.1 Get Test Tokens

1. Navigate to [Polkadot.js Apps](https://polkadot.js.org/apps/)
2. Connect to **Paseo Testnet**
3. Visit [Polkadot Faucet](https://faucet.polkadot.io/)
4. Request 100 PAS tokens (minimum recommended for deployment)

#### 1.2 Reserve Parachain ID

1. In Polkadot.js Apps, navigate to **Network > Parachains > Parathreads**
2. Click **"+ ParaId"** button
3. Submit the transaction (costs ~1 PAS)
4. Verify registration in **Explorer** - look for `registrar.Reserved` event
5. **Note your ParaId** - you'll need it for all subsequent steps

#### 1.3 Generate Collator Keys

**Generate Account Keys (Stash/Controller):**
```bash
# Generate stash account (sr25519)
subkey generate --scheme sr25519

# Save the output securely:
# - Secret phrase (mnemonic)
# - Public key (SS58 address)
# - Public key (hex)
```

**Generate Session Keys (Aura):**
```bash
# Generate Aura session key
subkey generate --scheme sr25519

# Save the output securely
```

**Important Security Notes:**
- Store account keys **offline** in a secure location
- Never share your secret phrases
- Session keys should be rotated regularly
- Use separate keys for stash and controller accounts

### Step 2: Build and Prepare Runtime

#### 2.1 Build Release Binary

```bash
# Ensure you're in the project root
cd /path/to/fintradex

# Build release version
cargo build --release

# Verify WASM runtime exists
ls -lh target/release/wbuild/fintradex-runtime/fintradex_runtime.compact.compressed.wasm
```

#### 2.2 Generate Chain Specification

**Generate Plain Chain Spec:**
```bash
chain-spec-builder --chain-spec-path ./fintradex_plain_chain_spec.json create \
  --relay-chain paseo \
  --para-id YOUR_PARA_ID \
  --runtime target/release/wbuild/fintradex-runtime/fintradex_runtime.compact.compressed.wasm \
  named-preset local_testnet
```

**Edit Chain Specification:**
Open `fintradex_plain_chain_spec.json` and update:

```json
{
  "name": "FintraDex Testnet",
  "id": "fintradex_testnet",
  "protocolId": "fintradex",
  "para_id": YOUR_PARA_ID,
  "parachainInfo": {
    "parachainId": YOUR_PARA_ID
  },
  "balances": {
    "balances": [
      ["YOUR_STASH_ADDRESS", "1000000000000000000000"]  // 1000 tokens
    ]
  },
  "collatorSelection": {
    "invulnerables": [
      "YOUR_COLLATOR_SS58_ADDRESS"
    ]
  },
  "session": {
    "keys": [
      [
        "YOUR_COLLATOR_SS58_ADDRESS",
        "YOUR_COLLATOR_SS58_ADDRESS",
        {
          "aura": "YOUR_AURA_PUBLIC_KEY_HEX"
        }
      ]
    ]
  },
  "sudo": {
    "key": "YOUR_SUDO_ACCOUNT_SS58_ADDRESS"
  }
}
```

**Convert to Raw Format:**
```bash
chain-spec-builder --chain-spec-path ./fintradex_raw_chain_spec.json convert-to-raw fintradex_plain_chain_spec.json
```

### Step 3: Export Genesis Files

#### 3.1 Export WASM Runtime

```bash
polkadot-omni-node export-genesis-wasm \
  --chain fintradex_raw_chain_spec.json \
  para-wasm
```

#### 3.2 Export Genesis State

```bash
polkadot-omni-node export-genesis-head \
  --chain fintradex_raw_chain_spec.json \
  para-state
```

**Verify Files:**
```bash
# Check file sizes (should be reasonable)
ls -lh para-wasm para-state

# WASM should be ~2-5MB
# State should be ~100-500KB
```

### Step 4: Register Parachain

#### 4.1 Upload Genesis Files

1. Navigate to [Polkadot.js Apps](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.paseo.io)
2. Connect to **Paseo Testnet**
3. Go to **Network > Parachains > Parathreads**
4. Click **"+ Parathread"** or **"+ Parachain"**
5. Upload files:
   - **Code**: Upload `para-wasm` file
   - **Initial State**: Upload `para-state` file
6. Submit transaction (requires sufficient PAS balance)

#### 4.2 Verify Registration

- Check **Explorer** for `registrar.Registered` event
- Verify your ParaId appears in the parachains list
- Wait for next session to start producing blocks

### Step 5: Start Collator Node

#### 5.1 Generate Node Key

```bash
polkadot-omni-node key generate-node-key \
  --base-path ./data \
  --chain fintradex_raw_chain_spec.json
```

**Note the node key location** (usually `./data/chains/fintradexidXXXX/network/secret_ed25519`)

#### 5.2 Start Collator

```bash
polkadot-omni-node \
  --collator \
  --chain fintradex_raw_chain_spec.json \
  --base-path ./data \
  --port 40333 \
  --rpc-port 9944 \
  --ws-port 9944 \
  --rpc-cors all \
  --name "YourCollatorName" \
  --node-key-file ./data/chains/fintradexidXXXX/network/secret_ed25519 \
  -- \
  --sync warp \
  --chain paseo \
  --execution wasm \
  --port 50343 \
  --rpc-port 9988 \
  --ws-port 9988
```

**Command Breakdown:**
- `--collator`: Run as collator
- `--chain`: Path to raw chain spec
- `--base-path`: Data directory
- `--port`: Parachain P2P port
- `--rpc-port`: Parachain RPC port
- `--ws-port`: Parachain WebSocket port
- `--node-key-file`: Node identity key
- `--`: Separator for relay chain arguments
- `--chain paseo`: Connect to Paseo relay chain
- `--sync warp`: Fast sync mode

#### 5.3 Verify Node Connection

```bash
# Check logs for:
# - "Parachain id: XXXX"
# - "Relay chain: paseo"
# - "Collator key: ..."
# - "Imported block #X"
```

### Step 6: Configure Session Keys

#### 6.1 Insert Session Keys

```bash
curl -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"author_insertKey",
    "params":[
      "aura",
      "YOUR_SECRET_PHRASE",
      "YOUR_PUBLIC_KEY_HEX"
    ],
    "id":1
  }' \
  http://localhost:9944
```

**Alternative: Use Polkadot.js Apps**
1. Connect to your local node (ws://localhost:9944)
2. Go to **Developer > RPC Calls**
3. Call `author.insertKey` with:
   - `keyType`: "aura"
   - `suri`: Your secret phrase
   - `publicKey`: Your public key hex

#### 6.2 Verify Session Keys

```bash
# Check session keys via RPC
curl -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"author_hasSessionKeys","params":["YOUR_PUBLIC_KEY_HEX"],"id":1}' \
  http://localhost:9944
```

### Step 7: Obtain Coretime

#### 7.1 Purchase Coretime (On-Demand)

1. Navigate to **Developer > Extrinsics**
2. Select account with PAS balance
3. Call `onDemand.placeOrderAllowDeath`
4. Set parameters:
   - `maxAmount`: Maximum PAS to spend (e.g., 1000000000000)
   - `paraId`: Your parachain ID
5. Submit transaction

#### 7.2 Verify Coretime Assignment

- Check **Network > Parachains** for your parachain status
- Verify blocks are being produced
- Monitor in **Explorer** for block production

---

### Getting Help

- **GitHub Issues**: [https://github.com/fintradev/fintradex/issues](https://github.com/fintradev/fintradex/issues)
- **Discussions**: [https://github.com/fintradev/fintradex/discussions](https://github.com/fintradev/fintradex/discussions)
- **Telegram**: [https://t.me/fintradex](https://t.me/fintradex)
- **Email**: team@fintradex.io

---

## Security Best Practices

### Key Management

1. **Never commit keys to git**
2. **Use environment variables for sensitive data**
3. **Rotate keys regularly**
4. **Use hardware security modules for production**
5. **Store backups encrypted and offline**

### Node Security

1. **Run nodes in isolated networks**
2. **Limit RPC access to trusted IPs**
3. **Use strong authentication**
4. **Keep software updated**
5. **Monitor for suspicious activity**

### Operational Security

1. **Follow principle of least privilege**
2. **Implement audit logging**
3. **Regular security reviews**
4. **Incident response plan**
5. **Regular backups**

---

## Additional Resources

- [Polkadot Documentation](https://docs.polkadot.network/)
- [Substrate Documentation](https://docs.substrate.io/)
- [Cumulus Documentation](https://docs.substrate.io/tutorials/connect-a-parachain/)
- [FintraDex Technical Whitepaper](./docs/Fintradex_Parachain_%20Technical_Architecture_Whitepaper.pdf)
- [Hyperbridge Documentation](https://docs.hyperbridge.network/)

---

**Last Updated**: 2025-11-06
**Maintained by**: FintraDex Team  
**For Support**: team@fintradex.io

