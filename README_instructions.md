<div align="center">

# FintradeX Parachain

> **The Future of Decentralized Financial Trading on Polkadot**

[![FintradeX](https://img.shields.io/badge/FintradeX-Financial%20Trading-blue?style=for-the-badge&logo=polkadot)](https://fintradex.io/)
[![Polkadot SDK](https://img.shields.io/badge/Polkadot%20SDK-Stable%202503-green?style=for-the-badge)](https://github.com/paritytech/polkadot-sdk)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

**A high-performance, cross-chain decentralized trading platform built on Polkadot**

</div>

## 🚀 Vision

FintradeX is revolutionizing decentralized finance by creating the most advanced trading infrastructure on Polkadot. We're building a platform that combines the speed and efficiency of traditional financial markets with the transparency and accessibility of blockchain technology.

### 🎯 Mission
- **Democratize Trading**: Make professional-grade trading tools accessible to everyone
- **Cross-Chain Liquidity**: Unify liquidity across multiple blockchain networks
- **Institutional-Grade Infrastructure**: Provide enterprise-level trading capabilities
- **Community-Driven Governance**: Empower users to shape the future of DeFi trading

## 🌟 Key Features

### 🏦 Advanced Trading Infrastructure
- **Multi-Asset Trading**: Trade any asset across multiple blockchains
- **High-Frequency Trading**: Sub-second order execution with minimal latency
- **Advanced Order Types**: Limit, market, stop-loss, and conditional orders
- **Real-Time Market Data**: Live price feeds and market analytics
- **Liquidity Pools**: Automated market making with deep liquidity

### 🔗 Cross-Chain Capabilities
- **Unified Trading Experience**: Trade assets from any connected blockchain
- **Seamless Asset Transfers**: Instant cross-chain asset movement
- **Interoperable Liquidity**: Share liquidity across the entire Polkadot ecosystem
- **Multi-Chain Order Books**: Unified order books across multiple networks

### 🛡️ Security & Compliance
- **Institutional-Grade Security**: Enterprise-level security protocols
- **Regulatory Compliance**: Built-in compliance features for institutional adoption
- **Audit-Ready Infrastructure**: Transparent and auditable trading operations
- **Risk Management**: Advanced risk controls and position monitoring

### 🏛️ Governance & Economics
- **DAO Governance**: Community-driven protocol decisions
- **Fee Distribution**: Fair and transparent fee sharing mechanisms
- **Staking Rewards**: Earn rewards by participating in network security
- **Treasury Management**: Community-controlled development funding

## 📊 Trading Features

### Spot Trading
- **Instant Settlement**: T+0 settlement for all trades
- **Deep Liquidity**: Access to liquidity across multiple chains
- **Advanced Charts**: Professional-grade trading charts and indicators
- **Portfolio Management**: Comprehensive portfolio tracking and analytics

### Derivatives Trading
- **Futures & Options**: Advanced derivative instruments
- **Leverage Trading**: Flexible margin trading with risk controls
- **Synthetic Assets**: Trade any asset as synthetic derivatives
- **Risk Hedging**: Advanced hedging strategies and tools

### DeFi Integration
- **Yield Farming**: Earn rewards through liquidity provision
- **Staking Protocols**: Participate in various staking opportunities
- **Lending & Borrowing**: Access to decentralized lending markets
- **Insurance Products**: Protect against smart contract risks

## 🏗️ Architecture

The FintradeX parachain consists of:

- 🧮 **[Runtime](./runtime/README.md)** - The core trading logic and state management
- 💿 **[Node](./node/README.md)** - High-performance blockchain node for trading operations
- 🔗 **Cross-Chain Bridge** - Seamless asset transfer between blockchains
- 📊 **Market Data Engine** - Real-time price feeds and market analytics
- 🛡️ **Security Layer** - Advanced security and risk management systems

## 🚀 Getting Started

### Prerequisites
- 🦀 **Rust**: 1.86 or higher
- 📦 **Cargo**: Latest version
- 🔧 **System Dependencies**: 
  - Ubuntu/Debian: `build-essential`, `cmake`, `pkg-config`, `libssl-dev`
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools

#### Rust Setup
Run the following commands to set up the correct Rust version:

```bash
rustup default 1.86
rustup target add wasm32-unknown-unknown --toolchain 1.86-aarch64-apple-darwin
rustup component add rust-src --toolchain 1.86-aarch64-apple-darwin
```

#### Required Tools

**Chain Spec Builder** - A Polkadot SDK utility for generating chain specifications. Refer to the [Generate Chain Specs documentation](https://docs.substrate.io/build/chain-spec/) for detailed usage.

Install it by executing the following command:

```bash
cargo install --locked staging-chain-spec-builder@10.0.0
```

This installs the `chain-spec-builder` binary.

**Polkadot Omni Node** - A white-labeled binary, released as a part of Polkadot SDK that can act as the collator of a parachain in production, with all the related auxiliary functionalities that a normal collator node has: RPC server, archiving state, etc. Moreover, it can also run the wasm blob of the parachain locally for testing and development.

To install it, run the following command:

```bash
cargo install --locked polkadot-omni-node@0.5.0
```

This installs the `polkadot-omni-node` binary.

### Technical Setup Guide

For advanced users and developers, follow these detailed steps to set up the parachain:

#### Development Mode Setup

##### 1. Clone the Repository
```bash
git clone https://github.com/fintradev/fintradex.git
cd fintradex
```

##### 2. Compile the Runtime
```bash
cargo build --release --locked
```

##### 3. Generate Development Chain Specification
Create a development network chain specification file:
```bash
chain-spec-builder create -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/fintradex-runtime/fintradex_runtime.compact.compressed.wasm \
  named-preset development
```

##### 4. Start Development Node
Start the node in development mode (without a relay chain config), producing and finalizing blocks:
```bash
polkadot-omni-node --chain ./chain_spec.json --dev
```

**Note**: This setup runs the parachain in standalone development mode for testing and development purposes.

### Parachain Deployment on Paseo Network

The following steps guide you through deploying your parachain on the Paseo testnet.

#### Prerequisites for Paseo Deployment

##### Account Preparation

To prepare an account, follow these steps:

1. **Open Polkadot.js Apps Interface**
   - Navigate to the [Polkadot.js Apps](https://polkadot.js.org/apps/) interface
   - Connect to the Paseo network
   - Navigate to the Accounts section

2. **Access Accounts**
   - Click on the Accounts tab in the top menu
   - Select the Accounts option from the dropdown menu

3. **Get Test Tokens**
   - Copy the address of the account you want to use for the parachain deployment from SubWallet after connecting with Paseo network
   - Visit the [Polkadot Faucet](https://faucet.polkadot.io/) and paste the copied address in the input field
   - Ensure that the network is set to Paseo and click on the "Get some PASs" button
   - You will receive 100 PAS tokens per request (available every 24 hours)

##### Reserve a Parachain Identifier

You must reserve a parachain identifier (ID) before registering your parachain on Paseo. You'll be assigned the next available identifier.

To reserve a parachain identifier, follow these steps:

1. **Navigate to Parachains Section**
   - Click on the Network tab in the top menu
   - Select the Parachains option from the dropdown menu

2. **Register a ParaId**
   - Select the Parathreads tab
   - Click on the "+ ParaId" button
   - Review the transaction and click on the "+ Submit" button

3. **Verify Registration**
   - After submitting the transaction, navigate to the Explorer tab
   - Check the list of recent events for successful `registrar.Reserved` event

##### Generate Collator Keys

To securely deploy your parachain, it is essential to generate custom keys specifically for your collators (block producers). You should generate two sets of keys for each collator:

- **Account keys** - Used to interact with the network and manage funds. These should be protected carefully and should never exist on the filesystem of the collator node
- **Session keys** - Used in block production to identify your node and its blocks on the network. These keys are stored in the parachain keystore and function as disposable "hot wallet" keys

**Security Note**: If session keys are leaked, someone could impersonate your node, which could result in the slashing of your funds. To minimize these risks, rotating your session keys frequently is essential. Treat them with the same level of caution as you would a hot wallet.

To generate keys, use `subkey`, a command-line tool for generating and managing keys:

```bash
# Generate account keys (sr25519)
subkey generate --scheme sr25519

# Generate session keys (sr25519 for Aura)
subkey generate --scheme sr25519
```

**Important**: Store your account keys securely offline and never share them. Session keys can be rotated regularly for enhanced security.

#### Paseo Deployment Steps

##### 1. Generate Plain Chain Spec
Create a plain chain specification file for Paseo deployment:
```bash
chain-spec-builder --chain-spec-path ./fintradex_plain_chain_spec.json create \
  --relay-chain paseo \
  --para-id 4866 \
  --runtime target/release/wbuild/fintradex-runtime/fintradex_runtime.compact.compressed.wasm \
  named-preset local_testnet
```

##### 2. Edit the Plain Chain Specification
Edit the `fintradex_plain_chain_spec.json` file:

- **Update the name, id, and protocolId fields** to unique values for your parachain
- **Change para_id and parachainInfo.parachainId fields** to the parachain ID you obtained previously. Make sure to use a number without quotes
- **Modify the balances field** to specify the initial balances for your accounts in SS58 format
- **Insert the account IDs and session keys** in SS58 format generated for your collators in the `collatorSelection.invulnerables` and `session.keys` fields
- **Modify the sudo value** to specify the account that will have sudo access to the parachain

##### 3. Generate Raw Chain Spec
Convert the modified plain chain specification file to raw format:
```bash
chain-spec-builder --chain-spec-path ./fintradex_raw_chain_spec.json convert-to-raw fintradex_plain_chain_spec.json
```

##### 4. Export Wasm Runtime
Export the Wasm runtime for the parachain by running the following command:
```bash
polkadot-omni-node export-genesis-wasm --chain fintradex_raw_chain_spec.json para-wasm
```

##### 5. Export Genesis State
Export the genesis state for the parachain by running the following command:
```bash
polkadot-omni-node export-genesis-head --chain fintradex_raw_chain_spec.json para-state
```

##### 6. Register Parachain on Paseo
Go to the **Parachains > Parathreads** tab, and select **+ Parathread**.

You should see fields to place your runtime Wasm and genesis state respectively, along with the parachain ID. Select your parachain ID, and upload:
- **para-wasm** in the code field
- **para-state** in the initial state field

##### 7. Generate Node Key
Before starting a collator, you need to generate a node key. This key is responsible for communicating with other nodes over Libp2p:

```bash
polkadot-omni-node key generate-node-key --base-path data --chain fintradex_raw_chain_spec.json
```

**Note**: Save both the generated key and the key path for reference: `data/chains/fintradexid4866/network/secret_ed25519`

##### 8. Start Collator Node
You must have the ports for the collator publicly accessible and discoverable to enable parachain nodes to peer with Paseo validator nodes to produce blocks. You can specify the ports with the `--port` command-line option.

You can start the collator with a command similar to the following:

```bash
polkadot-omni-node --collator --chain fintra_raw_chain_spec.json --base-path data --port 40333 --rpc-port 9944 --force-authoring --rpc-cors all --unsafe-rpc-external --rpc-methods=Unsafe 
--node-key-file ./data/chains/fintradexid4889/network/secret_ed25519 -- --sync warp --chain paseo --port 50343 --rpc-port 9988
```

##### 8.1. PM2 Process Management (Recommended for Production)
For production deployments, it's recommended to use PM2 for process management. This ensures your collator node automatically restarts if it crashes and provides better logging and monitoring.

**Install PM2:**
```bash
npm install -g pm2
```

**Start Collator with PM2:**
```bash
pm2 start bash --name "fintradex-collator" -- -c "polkadot-omni-node --collator --chain raw_chain_spec.json --base-path data --port 40333 --rpc-port 9944 --force-authoring --rpc-cors all --unsafe-rpc-external --rpc-methods=Unsafe --node-key-file ./data/chains/fintradexid4910/network/secret_ed25519 --state-pruning 512 --blocks-pruning 512 -- --chain paseo --port 50343 --rpc-port 9988 --sync warp --state-pruning 512 --blocks-pruning 512"
```

**PM2 Management Commands:**
```bash
# View all processes
pm2 list

# View logs
pm2 logs fintradex-collator

# Restart the collator
pm2 restart fintradex-collator

# Stop the collator
pm2 stop fintradex-collator

# Delete the process
pm2 delete fintradex-collator

# Save PM2 configuration
pm2 save

# Setup PM2 to start on system boot
pm2 startup
```

**PM2 Configuration File (ecosystem.config.js):**
```javascript
module.exports = {
  apps: [{
    name: 'fintradex-collator',
    script: 'polkadot-omni-node',
    args: '--collator --chain fintradex_raw_chain_spec.json --base-path data4890 --port 40333 --rpc-port 9944 --force-authoring --rpc-cors all --unsafe-rpc-external --rpc-methods=Unsafe --node-key-file ./data4890/chains/fintradexid4890/network/secret_ed25519 --state-pruning 512 --blocks-pruning 512 -- --chain paseo --port 50343 --rpc-port 9988 --sync warp --state-pruning 512 --blocks-pruning 512',
    instances: 1,
    autorestart: true,
    watch: false,
    max_memory_restart: '1G',
    env: {
      NODE_ENV: 'production'
    },
    log_file: './logs/fintradex-collator.log',
    out_file: './logs/fintradex-collator-out.log',
    error_file: './logs/fintradex-collator-error.log',
    log_date_format: 'YYYY-MM-DD HH:mm:ss Z'
  }]
}
```

**Start with configuration file:**
```bash
pm2 start ecosystem.config.js
```

##### 9. Insert Session Key into Collator Keystore
Before proceeding, ensure that the collator node is running. Then, open a new terminal and insert your generated session key into the collator keystore by running the following command. Use the same port specified in the `--rpc-port` parameter when starting the collator node (8845 in this example) to connect to it. Replace `INSERT_SECRET_PHRASE` and `INSERT_PUBLIC_KEY_HEX_FORMAT` with the values from the session key you generated in the **Generate Collator Keys** section:

```bash
curl -H "Content-Type: application/json" \
--data '{
  "jsonrpc":"2.0",
  "method":"author_insertKey",
  "params":[
    "aura",
    "INSERT_SECRET_PHRASE",
    "INSERT_PUBLIC_KEY_HEX_FORMAT"
  ],
  "id":1
}' \
http://localhost:8845
```
example 
curl http://localhost:9944 -H "Content-Type: application/json" --data '{"id":1,"jsonrpc":"2.0","method":"author_insertKey","params":["aura","liar arm dinosaur floor card van genuine chief fever artefact census such","0x2ae64f322586069ff85c0db88b4215555e87d1963cec73462c5a7660ddf25916"]}'
**Example:**
```bash
curl -H "Content-Type: application/json" \
--data '{"jsonrpc":"2.0","method":"author_insertKey","params":["aura","payment image magnet bicycle before public inner tail cover host already result","a8691d9613dba3487faad0570eb6329d9a00cff2149bce11af7ec1d705ac066b"],"id":1}' \
http://localhost:8845
```

**If successful, you should see the following response:**
```json
{"jsonrpc":"2.0","result":null,"id":1}
```

Once your collator is synced with the Paseo relay chain, and your parathread finished onboarding, it will be ready to start producing blocks. This process may take some time.

##### 10. Obtain Coretime

**Order On Demand Coretime**

There are two extrinsics which allow you to place orders for on-demand coretime:

- **`onDemand.placeOrderAllowDeath`** - will reap the account once the provided funds run out
- **`onDemand.placeOrderKeepAlive`** - includes a check that will not reap the account if the provided funds run out, ensuring the account is kept alive

To produce a block in your parachain, navigate to Polkadot.js Apps and ensure you're connected to the Paseo relay chain. Then, access the **Developer > Extrinsics** tab and execute the `onDemand.placeOrderAllowDeath` extrinsic from the account that registered the ParaID. 

For this example, `maxAmount` is set to `1000000000000` (this value may vary depending on the network conditions), and `paraId` is set to `4866`.