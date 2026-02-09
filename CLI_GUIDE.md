# Shera Blockchain CLI Documentation

The Shera Blockchain CLI (`shera`) provides a comprehensive command-line interface for interacting with the Shera network. This guide covers all available commands, their usage, and examples.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Global Options](#global-options)
- [Commands](#commands)
  - [Blockchain Initialization](#blockchain-initialization)
  - [Wallet Management](#wallet-management)
  - [Sending Transactions](#sending-transactions)
  - [Token Operations](#token-operations)
  - [Faucet](#faucet)
  - [Querying the Blockchain](#querying-the-blockchain)
  - [Deployment](#deployment)
- [Examples](#examples)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd crt-zkvm

# Build the CLI
cargo build --release

# The binary will be at target/release/crt-zkvm
# You can create a symlink for easier access:
ln -s $(pwd)/target/release/crt-zkvm /usr/local/bin/shera
```

### Using Cargo

```bash
# Install directly
cargo install --path .
```

## Quick Start

```bash
# Initialize the blockchain
shera init

# Check your balance
shera wallet balance

# Request test tokens from faucet
shera faucet --amount 1000

# Send tokens to another address
shera send --to <recipient-address> --amount 100 --token SHERA
```

## Global Options

These options can be used with any command:

```
-d, --db-path <PATH>     Path to database [default: ./data/shera.db]
-c, --config <PATH>      Path to config file [default: ./config.toml]
-h, --help               Print help
-V, --version            Print version
```

## Commands

### Blockchain Initialization

Initialize a new Shera blockchain instance.

```bash
shera init [OPTIONS]
```

**Options:**
- `--chain-id <ID>` - Chain identifier [default: shera-dev]

**Example:**
```bash
# Initialize with default settings
shera init

# Initialize with custom chain ID
shera init --chain-id my-testnet
```

**What happens:**
- Creates the database and data directory
- Creates a faucet wallet with 1,000,000,000 SHERA
- Creates a default wallet with 1,000,000 SHERA
- Initializes the genesis state

### Wallet Management

#### Create Wallet

Create a new wallet.

```bash
shera wallet create [OPTIONS]
```

**Options:**
- `-n, --name <NAME>` - Wallet name [default: default]

**Example:**
```bash
# Create wallet with default name
shera wallet create

# Create named wallet
shera wallet create --name alice
```

**Output:**
```
✓ Wallet 'alice' created successfully!
  Address: 0x1234...5678
  Public Key: 0xabcd...ef01

⚠️  IMPORTANT: Save your private key securely!
  Use 'shera wallet export --name alice' to view it.
```

#### List Wallets

List all wallets with their balances.

```bash
shera wallet list
```

**Output:**
```
Wallets:
NAME                 ADDRESS                                                          BALANCE (SHERA)
--------------------------------------------------------------------------------------------------------------
default              0x0000000000000000000000000000000000000000000000000000000000000001  1000000
alice                0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef  5000
faucet               0xfeed000000000000000000000000000000000000000000000000000000000000  999999000
```

#### Check Balance

Check wallet balance for all tokens or a specific token.

```bash
shera wallet balance [OPTIONS]
```

**Options:**
- `-w, --wallet <NAME>` - Wallet name or address [default: default]
- `-t, --token <SYMBOL>` - Token symbol (default: all tokens)

**Examples:**
```bash
# Check default wallet's SHERA balance
shera wallet balance

# Check specific wallet
shera wallet balance --wallet alice

# Check specific token
shera wallet balance --wallet alice --token MYTOKEN
```

**Output:**
```
Wallet: alice
Address: 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef

SHERA: 5000
```

#### Export Private Key

Export a wallet's private key (use with caution!).

```bash
shera wallet export [OPTIONS]
```

**Options:**
- `-n, --name <NAME>` - Wallet name [default: default]

**Example:**
```bash
shera wallet export --name alice
```

**Output:**
```
⚠️  WARNING: Never share your private key with anyone!

Wallet: alice
Address: 0x1234...5678

Private Key (hex): a1b2c3d4e5f6...
```

### Sending Transactions

Send Shera or custom tokens to another address.

```bash
shera send [OPTIONS] --to <ADDRESS> --amount <AMOUNT>
```

**Options:**
- `-t, --to <ADDRESS>` - Recipient address (required)
- `-a, --amount <AMOUNT>` - Amount to send (required)
- `--token <SYMBOL>` - Token symbol [default: SHERA]
- `--gas-limit <LIMIT>` - Gas limit [default: 21000]
- `--gas-price <PRICE>` - Gas price in gwei [default: 1]
- `-f, --from <NAME>` - Sender wallet name [default: default]

**Examples:**
```bash
# Send 100 SHERA
shera send --to 0x1234...5678 --amount 100

# Send with custom gas
shera send --to 0x1234...5678 --amount 100 --gas-limit 50000 --gas-price 2

# Send from specific wallet
shera send --to 0x1234...5678 --amount 100 --from alice
```

**Output:**
```
Sending 100 SHERA to 0x1234...5678...
✓ Transaction sent successfully!
  Amount: 100 SHERA
  To: 0x1234...5678
  Gas Used: 21000
  Tx Hash: 0xabcd...ef01
```

### Token Operations

#### Create Token

Create a new custom token (costs 1 SHERA).

```bash
shera token create [OPTIONS] --name <NAME> --symbol <SYMBOL>
```

**Options:**
- `-n, --name <NAME>` - Token name (required)
- `-s, --symbol <SYMBOL>` - Token symbol (required)
- `-d, --decimals <NUM>` - Number of decimals [default: 18]
- `--supply <AMOUNT>` - Initial supply [default: 1000000]
- `-f, --from <NAME>` - Creator wallet name [default: default]

**Example:**
```bash
shera token create \
  --name "My Token" \
  --symbol MTK \
  --decimals 18 \
  --supply 10000000 \
  --from alice
```

**Output:**
```
Creating token 'My Token' (MTK)...
  Decimals: 18
  Initial Supply: 10000000
  Creator: 0x1234...5678
✓ Token created successfully!
  Token ID: 12345
  Symbol: MTK
  Name: My Token
  Initial Supply: 10000000
```

#### List Tokens

List all registered tokens.

```bash
shera token list
```

**Output:**
```
Tokens:
ID         SYMBOL     NAME                           DECIMALS   SUPPLY
------------------------------------------------------------------------------------------
0          SHERA      Shera                          18         100000000
12345      MTK        My Token                       18         10000000
```

### Faucet

Request test Shera from the faucet.

```bash
shera faucet [OPTIONS]
```

**Options:**
- `-a, --address <ADDRESS>` - Address to fund (optional, uses default wallet if not specified)
- `--amount <AMOUNT>` - Amount to request [default: 1000]

**Examples:**
```bash
# Request 1000 SHERA to default wallet
shera faucet

# Request specific amount
shera faucet --amount 5000

# Fund specific address
shera faucet --address 0x1234...5678 --amount 10000
```

**Output:**
```
Requesting 1000 SHERA from faucet...
✓ Faucet transfer successful!
  Amount: 1000 SHERA
  To: 0x1234...5678
  Gas Used: 21000
  Tx Hash: 0xabcd...ef01
```

### Querying the Blockchain

#### Chain Status

Get current blockchain status.

```bash
shera query status
```

**Output:**
```
Shera Blockchain Status
=======================
Block Height: 0
State Root: 0xabcd...ef01

Default Wallet Balance: 1000000 SHERA
```

### Deployment

#### Deploy Template

Deploy a CRT template (placeholder).

```bash
shera deploy [OPTIONS] --code <PATH>
```

**Options:**
- `-c, --code <PATH>` - Path to template code file (required)
- `-k, --kind <TYPE>` - Type: template or contract [default: template]
- `-f, --from <NAME>` - Deployer wallet [default: default]
- `--gas-limit <LIMIT>` - Gas limit [default: 100000]

**Example:**
```bash
shera deploy --code ./my_template.bin --kind template
```

## Examples

### Complete Workflow Example

```bash
# 1. Initialize the blockchain
shera init --chain-id my-localnet

# 2. Create a new wallet
shera wallet create --name alice

# 3. Fund the wallet from faucet
shera faucet --address <alice-address> --amount 10000

# 4. Check balance
shera wallet balance --wallet alice

# 5. Send tokens to another address
shera send \
  --to 0x0000000000000000000000000000000000000000000000000000000000000002 \
  --amount 500 \
  --from alice

# 6. Create a custom token
shera token create \
  --name "Alice Token" \
  --symbol ALC \
  --decimals 18 \
  --supply 1000000 \
  --from alice

# 7. Check new token balance
shera wallet balance --wallet alice --token ALC
```

### Development Workflow

```bash
# Start a fresh development environment
rm -rf ./data
shera init

# Get test funds
shera faucet --amount 100000

# Deploy a contract (when implemented)
shera deploy --code ./contract.bin --kind contract

# Query status
shera query status
```

## Configuration

The CLI uses a TOML configuration file. Default location: `./config.toml`

### Example Configuration

```toml
[chain]
chain_id = "shera-dev"
genesis_alloc = [
    { address = "0x...", balance = "1000000000000000000000000" },
]

[storage]
db_path = "./data/shera.db"
wel_mode = true

[gas]
min_gas_price = "1000000000"  # 1 gwei
```

## Troubleshooting

### "Database not found" Error

**Solution:** Initialize the blockchain first:
```bash
shera init
```

### "Insufficient balance" Error

**Solution:** Request funds from faucet:
```bash
shera faucet --amount 1000
```

### "Wallet not found" Error

**Solution:** Create the wallet:
```bash
shera wallet create --name <wallet-name>
```

### Invalid Address Format

Addresses must be:
- 32 bytes (64 hex characters)
- Prefixed with `0x` or raw hex
- Example: `0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef`

### Gas Price Too Low

If your transaction fails with gas errors, increase the gas price:
```bash
shera send --to <address> --amount 100 --gas-price 10
```

## Security Best Practices

1. **Never share private keys**: Keep your wallet exports secure
2. **Use test networks**: For development, always use `--chain-id` with a test identifier
3. **Secure the data directory**: The `data/` folder contains sensitive wallet information
4. **Backup wallets**: Export and securely store private keys for important wallets

## Advanced Usage

### Scripting with the CLI

```bash
#!/bin/bash

# Create multiple wallets
for name in alice bob charlie; do
    shera wallet create --name $name
done

# Fund all wallets
for name in alice bob charlie; do
    address=$(shera wallet list | grep $name | awk '{print $2}')
    shera faucet --address $address --amount 10000
done
```

### Environment Variables

You can set default values via environment variables:

```bash
export SHERA_DB_PATH=/custom/path/to/shera.db
export SHERA_CONFIG=/custom/path/to/config.toml
```

## API Reference

For programmatic access, use the library directly:

```rust
use crt_zkvm::cli::CliHandler;

let handler = CliHandler::new(PathBuf::from("./data/shera.db"))?;
handler.wallet_create("alice")?;
```

## Support

For issues and questions:
- GitHub Issues: https://github.com/crt-zkvm/issues
- Documentation: https://docs.shera.network
- Discord: https://discord.gg/shera
