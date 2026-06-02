# Academy Vesting Contract

**Secure vesting and claim flows for academy rewards on Stellar Soroban**

![Vesting Timeline](https://img.shields.io/badge/Status-Production%20Ready-green)
![Tests](https://img.shields.io/badge/Tests-18%2B%20Passing-brightgreen)
![Coverage](https://img.shields.io/badge/Coverage-Comprehensive-brightgreen)

## 🎯 Overview

The **Academy Vesting Contract** is a Soroban smart contract that manages time-based vesting of academy rewards (badges, token incentives) with secure claim flows. It provides:

- ✅ **Time-Based Vesting**: Linear vesting after cliff periods
- ✅ **Single-Claim Semantics**: Atomic claims preventing double-spends
- ✅ **Governance Revocation**: Admin-controlled revocation with timelock protection
- ✅ **Event Emission**: Clear on-chain events for off-chain indexing
- ✅ **Comprehensive Security**: 5-layer security model with authorization checks
- ✅ **Production Tests**: 18+ comprehensive test cases

---

## 🚀 Quick Start

### Initialize Contract

```rust
AcademyVestingContract::init(
    env,
    admin_address,
    token_contract,
    governance_contract,
)?;
```

### Grant Vesting

```rust
let grant_id = AcademyVestingContract::grant_vesting(
    env,
    admin,
    user_address,
    5000,                        // 5000 tokens
    env.ledger().timestamp(),    // Start now
    86400,                       // 1-day cliff
    31536000,                    // 1-year total
)?;
```

### Claim Tokens

```rust
let claimed = AcademyVestingContract::claim(
    env,
    grant_id,
    user_address,
)?;
```

---

## 📋 Features

### Time-Based Vesting

- Configurable start time, cliff, and duration
- Linear vesting after cliff period
- Calculate vested amount at any time
- Support for any token amount

### Single-Claim Semantics

- Atomic claim operation (all-or-nothing)
- Prevents double-spends and replay attacks
- One claim per grant (no refunds)
- Clear error on re-attempt (AlreadyClaimed)

### Governance Revocation

- Admin-only revocation
- Minimum 1-hour timelock delay
- Cannot revoke claimed grants
- Clear revocation audit trail

### Event Emission

- GrantEvent: When vesting schedule created
- ClaimEvent: When tokens claimed
- RevokeEvent: When grant revoked
- Perfect for off-chain indexing

### Security

- Role-based authorization (Admin, Beneficiary)
- Input validation (cliff ≤ duration, amount > 0)
- Signature requirements for all state changes
- On-chain immutable history

---

## 🏗️ Architecture

### Data Structure

```rust
struct VestingSchedule {
    beneficiary: Address,    // Who receives tokens
    amount: i128,           // Total tokens
    start_time: u64,        // When vesting begins
    cliff: u64,             // Delay before unlocking
    duration: u64,          // Total vesting period
    claimed: bool,          // Single-claim flag
    revoked: bool,          // Revocation flag
    revoke_time: u64,       // When revoked
}
```

### Timeline

```
start_time ──cliff──> start_time+cliff ──linear vesting──> start_time+duration

Before Cliff:    0% vested
At Cliff:        0% vested
Midway:          ~50% vested
Full Duration:   100% vested
```

### Vesting Formula

```
vested_amount = amount × (elapsed_time / remaining_duration)
  where:
    elapsed_time = current_time - start_time - cliff
    remaining_duration = duration - cliff
```

---

## 🔐 Security

### 5-Layer Security Model

**Layer 1: Role-Based Authorization**

- Admin: grant, revoke
- Beneficiary: claim (with signature)
- Public: query

**Layer 2: Timelock Delays**

- Minimum 1-hour revocation delay
- Prevents surprise revocations
- User reaction window

**Layer 3: Atomic Operations**

- Claim is atomic (succeed or fail completely)
- Single-claim flag prevents replay
- No partial state changes

**Layer 4: State Machine**

- Clear vesting lifecycle
- Status transitions validated
- No invalid states possible

**Layer 5: Event Transparency**

- All actions emit events
- Off-chain indexing enabled
- Immutable audit trail

### Attack Prevention

| Attack             | Prevention                           |
| ------------------ | ------------------------------------ |
| Double-Claim       | Atomic `claimed` flag                |
| Replay             | Single-claim semantics + signature   |
| Unauthorized Claim | Beneficiary verification             |
| Unauthorized Grant | Admin verification                   |
| Surprise Revoke    | Timelock mechanism                   |
| Balance Drain      | Balance verification before transfer |

---

## 🧪 Testing

### Test Coverage: 18+ Tests

```
Initialization Tests (2)
├─ test_contract_initialization
└─ test_contract_cannot_be_initialized_twice

Grant Tests (4)
├─ test_grant_vesting_schedule
├─ test_grant_multiple_schedules
├─ test_grant_with_invalid_schedule
└─ test_non_admin_cannot_grant

Vesting Calculation Tests (5)
├─ test_vesting_calculation_before_start
├─ test_vesting_calculation_before_cliff
├─ test_vesting_calculation_after_cliff
├─ test_vesting_calculation_fully_vested
└─ test_vesting_calculation_partial

Claim Tests (4)
├─ test_claim_not_vested
├─ test_claim_single_semantics_prevents_double_claim
├─ test_claim_revoked_schedule
└─ test_claim_wrong_beneficiary

Revocation Tests (5)
├─ test_revoke_invalid_timelock
├─ test_revoke_not_enough_time_elapsed
├─ test_revoke_cannot_revoke_claimed
├─ test_revoke_cannot_revoke_twice
└─ test_non_admin_cannot_revoke

Query Tests (2)
├─ test_get_vesting_nonexistent
└─ test_get_vested_amount_nonexistent

Integration Test (1)
└─ test_integration_complete_vesting_flow
```

### Running Tests

```bash
cd Contracts/contracts/academy
cargo test --lib

# All 18+ tests pass ✅
```

---

## 📚 Documentation

| Document                                                   | Purpose                                    | Audience             |
| ---------------------------------------------------------- | ------------------------------------------ | -------------------- |
| [VESTING_DESIGN.md](./VESTING_DESIGN.md)                   | Complete technical design & architecture   | Developers, Auditors |
| [VESTING_QUICK_REFERENCE.md](./VESTING_QUICK_REFERENCE.md) | 5-minute quick start & cheat sheet         | Developers, DevOps   |
| [INTEGRATION_GUIDE.md](./INTEGRATION_GUIDE.md)             | Backend/frontend integration examples      | Full-Stack Engineers |
| [src/vesting.rs](./src/vesting.rs)                         | Smart contract implementation (600+ lines) | Developers           |
| [src/test.rs](./src/test.rs)                               | Comprehensive test suite (400+ lines)      | QA, Developers       |
| [Cargo.toml](./Cargo.toml)                                 | Rust package configuration                 | DevOps, Build        |

---

## 🔧 API Reference

### Core Functions

#### `init(env, admin, reward_token, governance)`

Initialize contract with admin and token addresses.

#### `grant_vesting(env, admin, beneficiary, amount, start_time, cliff, duration)`

Create vesting schedule (admin only).

#### `claim(env, grant_id, beneficiary)`

Claim vested tokens (atomic, single-claim).

#### `revoke(env, grant_id, admin, revoke_delay)`

Revoke grant with timelock (admin only).

#### `get_vesting(env, grant_id)`

Query vesting schedule details.

#### `get_vested_amount(env, grant_id)`

Calculate current vested amount.

#### `get_info(env)`

Get contract info (admin, token, governance).

---

## ⚠️ Error Codes

| Error                    | Code | When                  |
| ------------------------ | ---- | --------------------- |
| `Unauthorized`           | 4001 | Not admin/beneficiary |
| `NotVested`              | 4002 | Cliff not passed      |
| `AlreadyClaimed`         | 4003 | Already claimed once  |
| `InvalidSchedule`        | 4004 | Bad parameters        |
| `InsufficientBalance`    | 4005 | Not enough tokens     |
| `GrantNotFound`          | 4006 | ID doesn't exist      |
| `Revoked`                | 4007 | Grant revoked         |
| `InvalidTimelock`        | 4008 | Delay < 1 hour        |
| `NotEnoughTimeForRevoke` | 4009 | Timelock not elapsed  |

---

## 🚀 Deployment

### Build

```bash
cargo build --release --target wasm32-unknown-unknown
```

### Deploy to Testnet

```bash
soroban contract deploy \
  --network testnet \
  --source-account GXXXXXX \
  --wasm-file target/wasm32-unknown-unknown/release/academy_vesting.wasm
```

### Initialize

```bash
soroban contract invoke \
  --network testnet \
  --id [CONTRACT_ID] \
  --source-account [ADMIN] \
  -- init \
  --admin [ADMIN_ADDRESS] \
  --reward-token [TOKEN_ADDRESS] \
  --governance [GOVERNANCE_ADDRESS]
```

---

## 📊 Statistics

| Metric              | Value            |
| ------------------- | ---------------- |
| **Lines of Code**   | 600+             |
| **Test Cases**      | 18+              |
| **Test Coverage**   | Comprehensive    |
| **Security Layers** | 5                |
| **Error Types**     | 9                |
| **Events**          | 3 types          |
| **Functions**       | 7 core + 2 query |

---

## 🔗 Integration Points

### Backend

- Grant vesting when issuing academy rewards
- Store grant IDs in user profiles
- Monitor vesting status
- Handle revocations

### Frontend

- Display vesting progress
- Show claim eligibility
- Execute claims
- Track claim history

### Indexer

- Subscribe to Grant events
- Subscribe to Claim events
- Subscribe to Revoke events
- Build user vesting history

### Governance

- Monitor revocation events
- Track admin actions
- Audit grant history

---

## 💡 Usage Example

### Complete Flow

```rust
// 1. Initialize
AcademyVestingContract::init(env.clone(), admin, token, governance)?;

// 2. Backend grants vesting
let grant_id = AcademyVestingContract::grant_vesting(
    env.clone(),
    admin,
    user_address,
    5000,
    0,      // start_time (now)
    1000,   // cliff (1000 seconds)
    10000,  // duration (10000 seconds total)
)?;

// 3. Check vesting progress
let vested = AcademyVestingContract::get_vested_amount(env.clone(), grant_id)?;
println!("Vested: {}", vested);

// 4. User claims when fully vested
let claimed = AcademyVestingContract::claim(env.clone(), grant_id, user_address)?;
println!("Claimed: {}", claimed);

// 5. Cannot claim again
let error = AcademyVestingContract::claim(env, grant_id, user_address);
// Error: AlreadyClaimed (4003)
```

---

## ✅ Acceptance Criteria

- [x] **Time-based vesting** with cliff and duration
- [x] **Revocation with timelock** safety
- [x] **Single-claim semantics** (atomic, no double-spend)
- [x] **Event emission** (Grant, Claim, Revoke)
- [x] **Backend grant support** tied to wallets
- [x] **Comprehensive tests** covering all scenarios
- [x] **Integration test** demonstrating end-to-end flow

---

## 🔗 Related Contracts

- [Trading Contract](../trading/README.md) - Implements upgradeability with governance
- [Governance Module](../../shared/src/governance.rs) - Reusable governance for contracts

---

## 📞 Support

### Questions?

- **What is this?** → [VESTING_QUICK_REFERENCE.md](./VESTING_QUICK_REFERENCE.md)
- **How do I use it?** → [INTEGRATION_GUIDE.md](./INTEGRATION_GUIDE.md)
- **How does it work?** → [VESTING_DESIGN.md](./VESTING_DESIGN.md)
- **Show me code** → [src/vesting.rs](./src/vesting.rs)

### Issues

- Check [VESTING_DESIGN.md](./VESTING_DESIGN.md#⚠️-error-codes) for error code meanings
- Run tests: `cargo test --lib`
- Review test file for examples: [src/test.rs](./src/test.rs)

---

## 📄 License

Part of VitalisXAcademy Rewards System

---

## ✨ Highlights

✅ **Enterprise-Grade**: 5-layer security model  
✅ **Production-Ready**: 18+ comprehensive tests  
✅ **Well-Documented**: 2000+ lines of documentation  
✅ **User-Friendly**: Step-by-step integration guide  
✅ **Secure**: Single-claim + timelock prevents attacks  
✅ **Transparent**: All events on-chain for auditing  
✅ **Extensible**: Reusable for future academy programs

---

**Ready to deploy? Start with [VESTING_QUICK_REFERENCE.md](./VESTING_QUICK_REFERENCE.md)** 🚀
