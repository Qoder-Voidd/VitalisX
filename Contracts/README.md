# 📜 VitalisX Smart Contracts (Soroban)

Soroban smart contracts powering VitalisX, a Web3 crypto learning and social trading platform built on the Stellar blockchain. These contracts provide decentralized services for education credentials, social rewards, messaging, and on-chain trading used by the VitalisX backend and frontend applications.

This repository is intended for blockchain developers, protocol contributors, and the VitalisX platform infrastructure, serving as the trust layer for learning achievements, engagement rewards, user interactions, and decentralized trading features.

---

# 🆕 Upgradeability & Governance

**NEW**: All contracts now feature explicit upgradeability with on-chain governance support.

✅ **Multi-Signature Approval**: Upgrades require M-of-N approvals (e.g., 2-of-3)
✅ **Timelock Delays**: Prevents immediate execution (configurable: 1-24+ hours)
✅ **Role-Based Control**: Admin, Approver, and Executor roles prevent single points of failure
✅ **Transparent Governance**: All proposals tracked on-chain and auditable
✅ **Comprehensive Tests**: 10+ test cases covering all upgrade scenarios

## Documentation

* `UPGRADEABILITY.md` — Complete architecture & security analysis
* `GOVERNANCE_GUIDE.md` — Step-by-step upgrade procedures
* `QUICK_REFERENCE.md` — 30-second overview
* `IMPLEMENTATION_SUMMARY.md` — What was built

---

# Overview

This repository contains four core smart contracts that power the VitalisX ecosystem:

* **Trading Contract** (✨ Upgradeable): Decentralized exchange functionality for trading cryptocurrency pairs
* **Academy Contract**: Credential management for course completion and learning achievements
* **Social Rewards Contract**: Engagement tracking and reward distribution for community participation
* **Messaging Contract**: Decentralized messaging between users with read status tracking

---

# Project Structure

```txt
├── contracts/
│   ├── trading/         # ✨ Upgradeable DEX trading contract
│   ├── academy/         # ✨ NEW: Academy vesting & rewards contract
│   │   ├── VESTING_DESIGN.md
│   │   ├── VESTING_QUICK_REFERENCE.md
│   │   ├── INTEGRATION_GUIDE.md
│   │   ├── DELIVERY_SUMMARY.md
│   │   └── README.md
│   ├── social_rewards/  # Engagement rewards contract
│   └── messaging/       # P2P messaging contract
├── shared/
│   └── src/governance.rs
├── Cargo.toml
├── UPGRADEABILITY.md
├── GOVERNANCE_GUIDE.md
├── QUICK_REFERENCE.md
└── README.md
```

---

# Prerequisites

* Rust 1.70 or later
* Soroban SDK 20.5.0
* Stellar CLI tools

---

# Building

```bash
# Build all contracts
cargo build --release --target wasm32-unknown-unknown

# Build specific contract
cd contracts/trading
cargo build --release --target wasm32-unknown-unknown
```

---

# Testing

```bash
# Run all tests
cargo test --all

# Run specific contract tests
cd contracts/trading
cargo test
```

---

# Governance & Upgradeability

## Quick Start

All contracts support governance-controlled upgrades:

```bash
stellar contract invoke --id $CONTRACT_ID --source admin -- \
  init --admin $ADMIN --approvers [$A1,$A2,$A3] --executor $EXECUTOR
```

```bash
stellar contract invoke --id $CONTRACT_ID --source admin -- \
  propose_upgrade --new_contract_hash $HASH --description "..." \
  --approvers [$A1,$A2,$A3] --approval_threshold 2 --timelock_delay 3600
```

```bash
stellar contract invoke --id $CONTRACT_ID --source $APPROVER1 -- \
  approve_upgrade --proposal_id 1
```

```bash
stellar contract invoke --id $CONTRACT_ID --source $EXECUTOR -- \
  execute_upgrade --proposal_id 1
```

---

# Governance Features

* ✅ Multi-Sig Approval (M-of-N)
* ✅ Timelock Delays
* ✅ Role-Based Access Control
* ✅ Transparent Governance
* ✅ Circuit Breakers & Cancellation Support

---

# Deployment

## Testnet Deployment

### Configure Network

```bash
stellar config network set testnet https://soroban-testnet.stellar.org
```

### Configure RPC & Passphrase

```bash
stellar config set --scope global RPC_URL https://soroban-testnet.stellar.org
stellar config set --scope global NETWORK_PASSPHRASE "Test SDF Network ; September 2015"
```

### Build Contracts

```bash
cargo build --release --target wasm32-unknown-unknown
```

### Deploy Trading Contract

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/trading_contract.wasm \
  --source account-name \
  --network testnet
```

### Initialize Contract

```bash
stellar contract invoke \
  --id CONTRACT_ADDRESS \
  --source account-name \
  --network testnet \
  -- init \
  --admin "$ADMIN_ADDRESS" \
  --approvers '["$APPROVER1", "$APPROVER2", "$APPROVER3"]' \
  --executor "$EXECUTOR_ADDRESS"
```

---

# Contract Descriptions

## Trading Contract ✨ (Upgradeable)

Manages decentralized trading operations with governance support.

### Key Functions

* `init()`
* `trade()`
* `get_stats()`
* `propose_upgrade()`
* `approve_upgrade()`
* `execute_upgrade()`
* `pause()` / `unpause()`

---

## Academy Contract

Manages educational credentials, achievements, and vesting rewards.

### Features

#### Vesting Module

* `grant_vesting()`
* `claim()`
* `revoke()`
* `get_vesting()`
* `get_vested_amount()`

#### Credential Management

* `issue_credential()`
* `get_user_credentials()`
* `verify_credential()`

### Vesting Features

✅ Cliff periods
✅ Linear vesting
✅ Single-claim semantics
✅ Governance revocation
✅ Event emission
✅ Comprehensive tests

---

## Social Rewards Contract

Tracks engagement and distributes rewards.

### Key Functions

* `init()`
* `record_engagement()`
* `get_user_rewards()`
* `get_engagement_history()`
* `claim_tier_reward()`

---

## Messaging Contract

Enables decentralized peer-to-peer messaging.

### Key Functions

* `init()`
* `send_message()`
* `mark_as_read()`
* `get_messages()`
* `get_unread_count()`
* `get_stats()`

---

# Environment Variables

```bash
export STELLAR_SECRET_KEY="your-secret-key"

export SOROBAN_NETWORK="testnet"
export SOROBAN_RPC_URL="https://soroban-testnet.stellar.org"

export ADMIN_ADDRESS="G..."
export APPROVER_1="G..."
export APPROVER_2="G..."
export APPROVER_3="G..."
export EXECUTOR_ADDRESS="G..."
```

---

# Security Considerations

✅ Authentication via `require_auth()`
✅ Role-protected admin functions
✅ Safe instance storage management
✅ Governance-based upgrades
✅ Timelock protection
✅ Transparent proposal tracking

---

# Ecosystem Repositories

🌐 Frontend (Next.js): https://github.com/Dev-shamoo/VitalisX
⚙ Backend (NestJS): https://github.com/shamoo53/VitalisX_backend
⭐ Stellar Docs: https://developers.stellar.org/docs/smart-contracts/soroban/

---

# Contributing

🤝 Contributions are welcome.

## Workflow

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Pull latest changes to avoid conflicts
5. Submit a pull request

Please ensure:

* All tests pass
* Documentation is updated
* Code follows project standards

---

# Adding New Features

When adding new functionality:

1. Create a new function in the appropriate contract
2. Add corresponding tests
3. Update this README documentation
4. Ensure all tests pass before submitting

---

**Last Updated:** January 22, 2026
**Version:** 2.0 (Upgradeability & Governance)
**Status:** Production Ready
