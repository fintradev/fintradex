# Fintra

A decentralized exchange (DEX) built on Polkadot with an order book system. Fintra enables secure and efficient trading of digital assets with a familiar order book interface.

## Features

- Order Book System: Traditional order book interface for transparent price discovery
- Cross-Chain Trading: Leverage Polkadot's interoperability for trading assets across different parachains
- Secure Trading: Built on Polkadot's secure and scalable infrastructure
- High Performance: Optimized for fast order matching and execution
- User-Friendly: Familiar trading interface for both retail and institutional users

## Integrated Pallets

Below is a list of all integrated pallets in the Fintra runtime, along with their functionalities:

- **System**: Core functionality for the blockchain, including account management and block production.
- **Utility**: Provides utility functions for batch transactions and other common operations.
- **Timestamp**: Manages the current time for the blockchain.
- **Aura**: Handles block production and consensus.
- **Grandpa**: Provides finality gadget for the blockchain.
- **AssetRate**: Manages asset conversion rates.
- **Indices**: Handles account indices for better user experience.
- **Balances**: Manages native token balances of accounts.
- **TransactionPayment**: Handles transaction fees and payments.
- **Bounties**: Manages bounties for community contributions.
- **Assets**: Manages fungible and non-fungible assets.
- **PoolAssets**: Manages assets in liquidity pools.
- **Salary**: Manages salary payments for contributors.
- **CoreFellowship**: Manages core fellowship members and their roles.
- **VoterList**: Manages the list of voters for governance.
- **ChildBounties**: Manages child bounties for specific tasks.
- **Referenda**: Manages referenda for governance decisions.
- **AssetConversion**: Handles asset conversion operations.
- **RankedPolls**: Manages ranked polls for governance.
- **RankedCollective**: Manages ranked collective members.
- **FastUnstake**: Provides fast unstaking functionality.
- **Multisig**: Manages multi-signature transactions.
- **Vesting**: Manages token vesting schedules.
- **ElectionProviderMultiPhase**: Manages multi-phase elections.
- **Staking**: Manages staking operations and rewards.
- **Session**: Manages session keys for validators.
- **Council**: Manages the council for governance.
- **TechnicalMembership**: Manages technical membership roles.
- **TechnicalCommittee**: Manages the technical committee.
- **Preimage**: Manages preimages for governance proposals.
- **Treasury**: Manages the treasury for funding proposals.
- **Sudo**: Provides superuser functionality for the blockchain.
- **Historical**: Manages historical session data.
- **AssetConversionMigration**: Handles asset conversion migration.
- **Parameters**: Manages dynamic parameters for the runtime.
- **SkipFeelessPayment**: Allows skipping fees for certain transactions.
- **Whitelist**: Manages whitelisted addresses for specific operations.
- **Scheduler**: Manages scheduled tasks and operations.
- **ConvictionVoting**: Manages conviction voting for governance.
- **NominationPools**: Manages nomination pools for staking.

## Why Choose Fintra?

### Advantages Over Centralized Exchanges (CEX)

1. **Security & Control**
   - Users maintain full control of their assets (non-custodial)
   - No single point of failure or hacking vulnerability
   - No risk of exchange insolvency or exit scams
   - Private keys never leave user's control
   - Multi-signature wallet support for enhanced security
   - Time-locked transactions for large trades
   - Emergency withdrawal mechanisms

2. **Transparency**
   - All trades and order book data are on-chain and verifiable
   - No hidden fees or order manipulation
   - Real-time order book visibility
   - Transparent price discovery mechanism
   - Public audit trails for all transactions
   - Real-time settlement confirmation
   - Open-source smart contracts

3. **Accessibility**
   - No KYC requirements
   - Global access without restrictions
   - 24/7 trading without maintenance windows
   - Permissionless listing of assets
   - Mobile-first responsive interface
   - Multiple language support
   - Accessibility features for differently-abled users

4. **Cost Efficiency**
   - Lower trading fees compared to CEX
   - No withdrawal fees
   - No hidden spreads
   - Direct peer-to-peer trading
   - Volume-based fee discounts
   - Maker-taker fee model
   - Gas optimization for cost-effective trading

### Benefits of Order Book DEX

1. **Advanced Trading Features**
   - Limit orders for precise price execution
   - Stop-loss and take-profit orders
   - Market depth visualization
   - Price charts and technical analysis tools
   - Trailing stop orders
   - Iceberg orders for large trades
   - Time-in-force options (GTC, IOC, FOK)
   - Post-only orders to ensure maker status

2. **Professional Trading Experience**
   - Familiar interface for traditional traders
   - Advanced order types for sophisticated strategies
   - Real-time market data
   - Professional trading tools integration
   - Customizable trading interface
   - Multiple chart types and indicators
   - Trading view integration
   - API access for automated trading

3. **Liquidity Benefits**
   - Better price discovery through transparent order book
   - Reduced slippage for large orders
   - More efficient market making
   - Improved liquidity aggregation
   - Cross-chain liquidity pools
   - Automated market making integration
   - Liquidity provider incentives
   - Smart order routing

4. **Institutional Adoption**
   - Suitable for institutional trading
   - API support for algorithmic trading
   - High-frequency trading capabilities
   - Professional-grade infrastructure
   - Institutional-grade security features
   - Compliance tools and reporting
   - White-label solutions
   - Enterprise API access

5. **Cross-Chain Advantages**
   - Seamless trading across different parachains
   - Unified order book for cross-chain assets
   - Reduced fragmentation of liquidity
   - Enhanced market efficiency
   - Atomic cross-chain swaps
   - Bridge-less asset transfers
   - Cross-chain price feeds
   - Unified trading experience

6. **Technical Innovations**
   - Sub-second order matching engine
   - Parallel order processing
   - Optimized state management
   - Efficient memory usage
   - Low-latency trade execution
   - Scalable architecture
   - Real-time market data streaming
   - Advanced caching mechanisms

### Technical Implementation

1. **Polkadot Integration**
   - Built on Substrate framework for maximum flexibility
   - Parachain slot optimization for high throughput
   - Cross-chain message passing (XCMP) for asset transfers
   - Shared security through Polkadot's relay chain
   - Optimized runtime for order book operations
   - Custom pallets for DEX-specific functionality

2. **Order Matching Engine**
   - Price-time priority matching algorithm
   - Atomic order execution
   - Batch processing for high throughput
   - Memory-mapped order book for performance
   - Real-time order book updates
   - Smart order routing system
   - Anti-frontrunning mechanisms
   - MEV protection

3. **Smart Contract Architecture**
   - Modular design for easy upgrades
   - Optimized gas usage
   - Secure asset handling
   - Emergency pause functionality
   - Upgradeable contracts
   - Comprehensive event logging
   - Access control mechanisms

### Governance System

1. **On-Chain Governance**
   - Token holder voting rights
   - Proposal submission and voting
   - Treasury management
   - Parameter adjustment capabilities
   - Emergency proposals
   - Council elections
   - Technical committee

2. **Protocol Parameters**
   - Fee structure adjustment
   - Trading pair management
   - Asset listing criteria
   - Security parameters
   - Performance optimization
   - Cross-chain parameters
   - Emergency controls

### Token Economics

1. **Utility Token (PDEX)**
   - Trading fee discounts
   - Governance voting power
   - Staking rewards
   - Liquidity mining incentives
   - Protocol fee sharing
   - Cross-chain bridge fees
   - Validator rewards

2. **Token Distribution**
   - Community allocation
   - Development fund
   - Ecosystem growth
   - Team and advisors
   - Marketing and partnerships
   - Liquidity mining
   - Treasury reserve

3. **Economic Model**
   - Deflationary tokenomics
   - Fee burning mechanism
   - Staking rewards
   - Liquidity incentives
   - Cross-chain rewards
   - Governance incentives
   - Protocol-owned liquidity

## Development Setup Guide

### Prerequisites

1. **Rust Setup**
   ```bash
   # Install Rust 1.86.0
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   source $HOME/.cargo/env
   rustup default 1.86.0
   rustup update
   
   # Add WASM target
   rustup target add wasm32-unknown-unknown
   
   # Install required components
   rustup component add rustfmt
   rustup component add clippy
   rustup component add rust-src
   rustup component add rust-analysis
   rustup component add rls
   ```

2. **System Dependencies**
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev

   # macOS
   brew install cmake pkg-config openssl git llvm
   ```

3. **Node.js and Yarn** (for frontend development)
   ```bash
   # Using nvm (recommended)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   source ~/.bashrc  # or source ~/.zshrc
   nvm install 18
   nvm use 18
   
   # Install Yarn
   npm install -g yarn
   ```

### Building from Source

1. **Clone the Repository**
   ```bash
   git clone https://github.com/wasif1024/fintra.git
   cd fintra
   ```

2. **Build the Project**
   ```bash
   # Build the node
   cargo build --release
   
   # Build the runtime
   cargo build --release -p fintra-runtime
   ```

3. **Run Tests**
   ```bash
   # Run all tests
   cargo test --all
   
   # Run specific test
   cargo test -p fintra-runtime
   ```

### Running the Node

1. **Development Mode**
   ```bash
   # Start a development node
   ./target/release/fintra-node --dev
   
   # Start with specific base path
   ./target/release/fintra-node --dev --base-path ./my-chain-state
   ```

2. **Production Mode**
   ```bash
   # Start a production node
   ./target/release/fintra-node --chain mainnet
   ```

### Development Tools

1. **Useful Cargo Commands**
   ```bash
   # Check code formatting
   cargo fmt --all
   
   # Run linter
   cargo clippy
   
   # Check for unused dependencies
   cargo udeps
   
   # Update dependencies
   cargo update
   ```

2. **Development Dependencies**
   ```bash
   # Install development tools
   cargo install cargo-udeps
   cargo install cargo-deny
   cargo install cargo-audit
   cargo install cargo-nextest
   cargo install cargo-watch
   ```

### Troubleshooting

1. **Common Issues**
   - If you encounter build errors, try:
     ```bash
     cargo clean
     cargo update
     cargo build
     ```
   - For WASM build issues:
     ```bash
     rustup target add wasm32-unknown-unknown
     cargo clean
     cargo build
     ```

2. **Memory Issues**
   - If you encounter memory issues during build:
     ```bash
     export RUSTFLAGS="-C target-cpu=native"
     cargo build --release
     ```

3. **Network Issues**
   - If you have trouble connecting to the network:
     ```bash
     # Check your network configuration
     ./target/release/fintra-node --dev --ws-external
     ```

### Contributing

1. **Development Workflow**
   ```bash
   # Create a new branch
   git checkout -b feature/your-feature-name
   
   # Make your changes
   # Run tests
   cargo test
   
   # Format code
   cargo fmt
   
   # Create pull request
   git push origin feature/your-feature-name
   ```

2. **Code Quality**
   - Run all tests before submitting PR
   - Ensure code is properly formatted
   - Check for clippy warnings
   - Update documentation if needed

### Additional Resources

- [Polkadot Documentation](https://wiki.polkadot.network/)
- [Substrate Documentation](https://docs.substrate.io/)
- [Rust Documentation](https://doc.rust-lang.org/book/)
- [Fintra API Documentation](./docs/api.md)

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo
- Git

### Installation

1. Clone the repository:
```sh
git clone https://github.com/wasif1024/fintra.git
cd fintra
```

2. Build the project:
```sh
cargo build --release
```

### Running the Node

To start a development node:

```sh
./target/release/fintra-node --dev
```

To start with a specific base path for persistent storage:

```sh
./target/release/fintra-node --dev --base-path ./my-chain-state
```

### Development Chain Features

- Maintains state in a `tmp` folder while running
- Uses pre-configured development accounts for testing
- Includes a genesis state with pre-funded accounts for development

### Interacting with Fintra

You can interact with Fintra using:
- Polkadot.js Apps (connect to ws://localhost:9944)
- Direct API calls to the node
- Custom trading interfaces

## Project Structure

### Node (`/node`)

The node implementation includes:
- Network communication
- Consensus mechanisms
- RPC server for external interactions
- Chain specification for genesis state

### Runtime (`/runtime`)

The runtime contains the core business logic:
- Order book management
- Trading engine
- Asset management
- Cross-chain integration

### Pallets

The project includes several custom pallets:
- Order Book Pallet: Manages the order book system
- Trading Pallet: Handles order matching and execution
- Asset Pallet: Manages asset registration and transfers
- Liquidity Pallet: Handles liquidity provision and management

## Development

### Building Documentation

Generate and view the Rust documentation:

```sh
cargo +nightly doc --open
```

### Testing

Run the test suite:

```sh
cargo test
```

## Contributing

We welcome contributions! Please see our contributing guidelines for more details.

## License

This project is licensed under the terms of the MIT license.

### Connecting with Polkadot.js Apps

1. **Start Your Local Node**
   ```bash
   # Start the node in development mode with external WebSocket
   ./target/release/fintra-node --dev --ws-external
   ```

2. **Access Polkadot.js Apps**
   - Open [Polkadot.js Apps](https://polkadot.js.org/apps/#/explorer)
   - Click on the network selector in the top-left corner
   - Click on "Development" section
   - Click on "Local Node" (ws://127.0.0.1:9944)

3. **Configure Custom Endpoint**
   If the local node is not automatically detected:
   - Click on the network selector
   - Click on "Development" section
   - Click on "Custom Endpoint"
   - Enter: `ws://127.0.0.1:9944`
   - Click "Save"

4. **Using the Interface**
   - **Explorer**: View blocks, extrinsics, and events
   - **Accounts**: Manage your accounts and balances
   - **Extrinsics**: Submit transactions
   - **Chain State**: Query on-chain state
   - **Settings**: Configure interface preferences

5. **Development Accounts**
   The following development accounts are pre-funded:
   ```
   Alice: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
   Bob: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
   Charlie: 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
   ```

6. **Common Operations**
   - **Create Account**: 
     - Go to "Accounts" → "Accounts"
     - Click "Add Account"
     - Follow the account creation wizard
   
   - **Transfer Tokens**:
     - Go to "Accounts" → "Transfer"
     - Select sender and recipient
     - Enter amount and submit

   - **View Order Book**:
     - Go to "Chain State"
     - Select "orderBook" pallet
     - Query current orders

7. **Troubleshooting Connection**
   - Ensure your node is running with `--ws-external` flag
   - Check if port 9944 is not blocked by firewall
   - Verify WebSocket connection in browser console
   - Try using `ws://localhost:9944` if `127.0.0.1` doesn't work

8. **Development Tips**
   - Use the "Settings" → "Developer" section to view type definitions
   - Enable "Developer" mode in settings for advanced features
   - Use the "RPC" section to make direct API calls
   - Monitor events in the "Explorer" section

9. **Security Considerations**
   - Never use development accounts in production
   - Keep your private keys secure
   - Use different accounts for different purposes
   - Regularly backup your accounts

10. **Additional Tools**
    - [Polkadot.js Extension](https://polkadot.js.org/extension/): Browser extension for account management
    - [Polkadot.js API](https://polkadot.js.org/api/): JavaScript API for custom applications
    - [Polkadot.js Common](https://polkadot.js.org/common/): Utility functions and helpers

### Frontend User Guide

1. **Trading Interface**
   - **Order Book Display**
     - Real-time order book visualization
     - Price depth chart
     - Recent trades feed
     - Order entry panel
   
   - **Trading Pairs**
     - Select trading pairs from the dropdown
     - View pair statistics and charts
     - 24h volume and price change
     - Current best bid/ask prices

2. **Order Types**
   - **Limit Orders**
     - Set specific price for buying/selling
     - Post-only option to ensure maker status
     - Time-in-force options (GTC, IOC, FOK)
   
   - **Market Orders**
     - Immediate execution at best available price
     - Slippage protection settings
     - Maximum price impact warning
   
   - **Advanced Orders**
     - Stop-loss orders
     - Take-profit orders
     - Trailing stop orders
     - Iceberg orders for large trades

3. **Account Management**
   - **Wallet Connection**
     - Connect with Polkadot.js Extension
     - View account balance
     - Transaction history
     - Open orders
   
   - **Security Features**
     - Two-factor authentication
     - Session management
     - API key management
     - Withdrawal whitelist

4. **Trading Features**
   - **Chart Analysis**
     - Multiple timeframes (1m to 1d)
     - Technical indicators
     - Drawing tools
     - Price alerts
   
   - **Portfolio Management**
     - Asset allocation view
     - Performance tracking
     - PnL calculator
     - Trade history export

5. **User Preferences**
   - **Interface Customization**
     - Dark/Light theme
     - Layout customization
     - Chart color schemes
     - Order book depth levels
   
   - **Trading Settings**
     - Default order type
     - Slippage tolerance
     - Confirmation dialogs
     - Sound notifications

6. **Mobile Experience**
   - **Responsive Design**
     - Optimized for all screen sizes
     - Touch-friendly interface
     - Mobile-specific features
     - Offline mode support
   
   - **Mobile Features**
     - Biometric authentication
     - Push notifications
     - Quick trade buttons
     - Mobile-optimized charts

7. **Trading Tools**
   - **Market Analysis**
     - Order book heatmap
     - Volume profile
     - Market depth analysis
     - Historical data
   
   - **Trading Bots**
     - Basic bot templates
     - Custom strategy creation
     - Backtesting tools
     - Performance analytics

8. **Help & Support**
   - **Documentation**
     - Interactive tutorials
     - Video guides
     - FAQ section
     - API documentation
   
   - **Support Channels**
     - Live chat support
     - Ticket system
     - Community forums
     - Social media channels

9. **Best Practices**
   - **Security**
     - Use hardware wallets for large amounts
     - Enable 2FA
     - Regular security audits
     - Secure API key management
   
   - **Trading**
     - Start with small orders
     - Use limit orders for better prices
     - Monitor market depth
     - Set stop-loss orders

10. **Advanced Features**
    - **API Integration**
      - REST API access
      - WebSocket streams
      - Rate limits and quotas
      - API key management
    
    - **Institutional Features**
      - OTC trading desk
      - Block trading
      - Custom settlement options
      - White-label solutions
