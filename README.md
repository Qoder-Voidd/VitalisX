# 🏥 VitalisX — Decentralized Patient Health Records on Stellar

> **A sovereign, privacy-first healthcare records platform built on the Stellar blockchain — enabling healthcare professionals to securely record, access, and share patient illness histories with full auditability, patient consent management, and on-chain data integrity.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built on Stellar](https://img.shields.io/badge/Built%20on-Stellar-blueviolet)](https://stellar.org)
[![Soroban Smart Contracts](https://img.shields.io/badge/Smart%20Contracts-Soroban-orange)](https://soroban.stellar.org)
[![Monorepo](https://img.shields.io/badge/Monorepo-pnpm%20workspaces-yellow)](https://pnpm.io/workspaces)

---

## 📌 Table of Contents

- [Overview](#overview)
- [Problem Statement](#problem-statement)
- [Why Stellar](#why-stellar)
- [Features](#features)
- [Monorepo Structure](#monorepo-structure)
- [Architecture](#architecture)
- [Smart Contracts (Soroban)](#smart-contracts-soroban)
- [Apps](#apps)
  - [Web App (Frontend)](#web-app-frontend)
  - [API / Backend](#api--backend)
  - [Mobile App](#mobile-app)
- [Packages](#packages)
- [Getting Started](#getting-started)
- [Environment Variables](#environment-variables)
- [Database Schema](#database-schema)
- [Consent & Privacy Model](#consent--privacy-model)
- [Tokenomics & Incentives](#tokenomics--incentives)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

**VitalisX** is a decentralized, privacy-first Electronic Health Records (EHR) platform built on the **Stellar blockchain** using **Soroban smart contracts**. It enables healthcare professionals — doctors, nurses, pharmacists, and hospitals — to record, retrieve, and share patient illness histories in a secure, tamper-proof, and auditable manner.

Unlike traditional EHR systems that are siloed within individual hospitals, VitalisX creates a **unified, patient-centric health graph** where the patient owns their data and grants time-scoped, role-scoped consent to healthcare providers — all enforced on-chain.

This project is an open-source public good for the global health sector.

---

## Problem Statement

Healthcare systems globally suffer from:

- **Fragmented records** — a patient's history is scattered across multiple hospitals with no interoperability
- **No patient data sovereignty** — patients have no control over who accesses their records
- **Tampering risks** — records stored in centralized databases can be altered without audit trails
- **Delayed access in emergencies** — a patient transferred to a new hospital has no records available
- **Medication duplication** — without a unified prescription history, dangerous drug interactions go undetected
- **Opaque billing** — patients cannot verify what procedures were billed

VitalisX solves every one of these problems using Stellar's fast, low-cost, and programmable blockchain infrastructure.

---

## Why Stellar

VitalisX specifically targets the **Stellar blockchain** for the following reasons:

- **Transaction throughput** — 5-second finality with ~4,000 TPS, ideal for high-frequency medical record entries
- **Low fees** — sub-cent transaction costs make micro-interactions (e.g., logging a vitals check) economically viable
- **Soroban smart contracts** — Rust-based, auditable, deterministic contract execution for consent and access control
- **USDC / stablecoin support** — enables healthcare payment settlement natively on the same chain
- **Stellar Quest ecosystem** — established developer tooling and grant infrastructure
- **Energy efficiency** — SCP consensus (not Proof-of-Work), environmentally responsible for a public-good application

---

## Features

### 🔐 Core Healthcare Features
- **Patient Health Records (PHR)** — Create and maintain structured illness histories, diagnoses, treatments, and outcomes
- **Multi-provider access** — Doctors, nurses, pharmacists, labs, and specialists each have role-appropriate views
- **Emergency access mode** — A patient can generate a time-limited emergency QR code for unconscious/emergency scenarios
- **Medication history & interaction checker** — Full prescription ledger with on-chain drug-interaction flagging
- **Lab results & imaging links** — IPFS-linked diagnostic results attached to on-chain record hashes
- **Allergy registry** — Immutable allergy and adverse-reaction records across providers
- **Vaccination ledger** — Verifiable vaccination history usable for travel, school, and employment verification
- **Referral management** — On-chain specialist referral requests with status tracking

### 🔑 Consent & Privacy
- **Patient-controlled consent** — Patients grant/revoke access to specific providers via on-chain transactions
- **Granular consent scopes** — Consent can be limited to specific record categories (e.g., "this cardiologist sees only heart records")
- **Time-bounded access** — All consent grants have expiry timestamps enforced by Soroban contracts
- **Consent audit trail** — Every access event is logged on-chain, providing a full audit log the patient can review
- **Data encryption** — All off-chain record content is encrypted with patient-derived keys; only hash/metadata go on-chain

### 💊 Clinical Decision Support
- **Drug interaction alerts** — Real-time medication cross-checking using an embedded drug database
- **Chronic condition flags** — Automatic highlighting of patients with chronic conditions during intake
- **Duplicate prescription detection** — Prevents double-prescribing across different providers
- **AI-powered symptom summarizer** — Summarizes a patient's multi-year history into a brief clinical snapshot for new providers

### 💳 Healthcare Finance (On-Chain)
- **Medical billing ledger** — Transparent, line-item billing records linked to treatment entries
- **Insurance claim anchoring** — Bill hashes anchored on Stellar for dispute resolution
- **Micro-payment settlements** — Direct patient-to-provider stablecoin (USDC) payments for consultations
- **Pharmaceutical supply chain** — Track medication provenance from manufacturer to dispense event

### 🏥 Institutional Features
- **Multi-facility network** — Hospitals, clinics, and labs join a permissioned network with on-chain attestation
- **Staff credentialing** — Healthcare professional licenses are stored as verifiable credentials on Stellar
- **Analytics dashboard** — Aggregate (anonymized) public health trends by region, disease, and demographic
- **Appointment scheduling** — On-chain appointment booking with tokenized slot NFTs (no-show penalties enforced via Soroban)

---

## Monorepo Structure

```
vitalisx/
├── apps/
│   ├── web/                    # Next.js 14 web application (healthcare professionals + patients)
│   ├── api/                    # NestJS REST & GraphQL API server
│   ├── mobile/                 # React Native patient-facing mobile app
│   └── admin/                  # Internal admin dashboard (facility onboarding, analytics)
│
├── contracts/
│   ├── patient-registry/       # Soroban: Patient identity and record root registration
│   ├── consent-manager/        # Soroban: Consent grants, revocations, access control
│   ├── record-anchor/          # Soroban: Record hash anchoring and integrity verification
│   ├── credential-vault/       # Soroban: Healthcare professional credential attestation
│   ├── billing-ledger/         # Soroban: Transparent medical billing and claim anchoring
│   └── appointment-token/      # Soroban: Tokenized appointment slots
│
├── packages/
│   ├── ui/                     # Shared component library (Radix UI + Tailwind)
│   ├── stellar-sdk-utils/      # Stellar/Soroban client abstractions and helpers
│   ├── crypto/                 # Patient key derivation, encryption/decryption utilities
│   ├── drug-db/                # Embedded drug interaction database (RxNorm-compatible)
│   ├── types/                  # Shared TypeScript types across apps
│   ├── config/                 # Shared ESLint, Prettier, TypeScript configs
│   └── validators/             # Zod schemas for clinical data structures
│
├── docs/                       # Architecture docs, ADRs, API reference
├── scripts/                    # Deployment scripts, contract migration tools
├── docker-compose.yml          # Local development environment
├── pnpm-workspace.yaml
├── turbo.json
└── README.md
```

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                        CLIENT LAYER                               │
│   ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│   │  Web App     │  │ Mobile App   │  │   Admin Dashboard    │  │
│   │ (Next.js 14) │  │(React Native)│  │     (Next.js 14)     │  │
│   └──────┬───────┘  └──────┬───────┘  └──────────┬───────────┘  │
└──────────┼─────────────────┼─────────────────────┼──────────────┘
           │                 │                       │
           └────────────────►│◄──────────────────────┘
                             ▼
┌──────────────────────────────────────────────────────────────────┐
│                        API LAYER                                  │
│              NestJS — REST + GraphQL + WebSocket                  │
│         Auth (Stellar wallet), RBAC, Rate limiting               │
│     Supabase (PostgreSQL) ── IPFS/Filebase (encrypted files)     │
└──────────────────────┬───────────────────────────────────────────┘
                        │
           ┌────────────┼────────────┐
           ▼            ▼            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    STELLAR BLOCKCHAIN LAYER                       │
│                                                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │ patient-registry│  │ consent-manager │  │  record-anchor  │  │
│  │   (Soroban)     │  │   (Soroban)     │  │   (Soroban)     │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │credential-vault │  │ billing-ledger  │  │  appt-token     │  │
│  │   (Soroban)     │  │   (Soroban)     │  │   (Soroban)     │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Data Flow for Record Creation:**
1. Healthcare professional authenticates via their Stellar wallet (Freighter / Lobstr)
2. API validates their credential via the `credential-vault` contract
3. Record is encrypted client-side with patient's derived symmetric key
4. Encrypted blob is uploaded to IPFS; the content hash is returned
5. API calls `record-anchor` contract to store `(patient_id, provider_id, ipfs_hash, record_type, timestamp)` on-chain
6. Consent check is performed in `consent-manager` before any read operation

---

## Smart Contracts (Soroban)

All contracts are written in **Rust** targeting Soroban (Stellar's smart contract platform).

### `patient-registry`
Manages patient identity registration and their record root.

```rust
pub trait PatientRegistry {
    fn register_patient(env: Env, patient_stellar_id: Address, metadata_hash: BytesN<32>) -> Result<u64, Error>;
    fn get_patient(env: Env, patient_id: u64) -> Result<PatientRecord, Error>;
    fn update_metadata(env: Env, patient_id: u64, new_hash: BytesN<32>) -> Result<(), Error>;
    fn deactivate_patient(env: Env, patient_id: u64) -> Result<(), Error>;
}
```

### `consent-manager`
Core privacy enforcement — every record read goes through this contract.

```rust
pub trait ConsentManager {
    fn grant_consent(env: Env, patient_id: u64, provider_address: Address, scope: ConsentScope, expires_at: u64) -> Result<ConsentId, Error>;
    fn revoke_consent(env: Env, patient_id: u64, consent_id: ConsentId) -> Result<(), Error>;
    fn check_access(env: Env, patient_id: u64, provider_address: Address, record_type: RecordType) -> Result<bool, Error>;
    fn list_active_consents(env: Env, patient_id: u64) -> Vec<ConsentGrant>;
    fn list_access_log(env: Env, patient_id: u64, limit: u32) -> Vec<AccessEvent>;
}
```

### `record-anchor`
Immutable anchoring of record hashes — the single source of truth for record integrity.

```rust
pub trait RecordAnchor {
    fn anchor_record(env: Env, patient_id: u64, provider_id: u64, ipfs_hash: String, record_type: RecordType) -> Result<RecordId, Error>;
    fn verify_record(env: Env, record_id: RecordId, ipfs_hash: String) -> bool;
    fn get_record_chain(env: Env, patient_id: u64, record_type: Option<RecordType>) -> Vec<AnchoredRecord>;
    fn flag_record(env: Env, record_id: RecordId, reason: String) -> Result<(), Error>;
}
```

### `credential-vault`
Verifiable credentials for healthcare professionals.

```rust
pub trait CredentialVault {
    fn issue_credential(env: Env, professional_address: Address, credential_type: CredentialType, issuer: Address, expiry: u64) -> Result<CredentialId, Error>;
    fn verify_credential(env: Env, professional_address: Address, required_type: CredentialType) -> bool;
    fn revoke_credential(env: Env, credential_id: CredentialId) -> Result<(), Error>;
}
```

### `billing-ledger`
Transparent, on-chain medical billing.

```rust
pub trait BillingLedger {
    fn create_bill(env: Env, patient_id: u64, provider_id: u64, line_items: Vec<BillItem>) -> Result<BillId, Error>;
    fn anchor_claim(env: Env, bill_id: BillId, insurer_address: Address) -> Result<ClaimId, Error>;
    fn settle_payment(env: Env, bill_id: BillId, amount: i128, token: Address) -> Result<(), Error>;
    fn dispute_bill(env: Env, bill_id: BillId, reason: String) -> Result<DisputeId, Error>;
}
```

---

## Apps

### Web App (Frontend)

**Path:** `apps/web`  
**Stack:** Next.js 14, TypeScript, Tailwind CSS, Radix UI, SWR, Stellar Freighter wallet

**Key Screens:**
- `/dashboard` — Provider home with recent patients and pending tasks
- `/patients` — Patient search and registration
- `/patients/[id]` — Full patient health timeline
- `/patients/[id]/records/new` — Structured record entry form (diagnosis, treatment, prescriptions)
- `/consent` — Patient consent management portal
- `/prescriptions` — Prescription pad with drug interaction checker
- `/labs` — Lab result upload and IPFS linking
- `/billing` — Bill creation and claim anchoring
- `/credentials` — Provider license management

```bash
cd apps/web
pnpm dev         # http://localhost:3000
pnpm build
pnpm lint
```

### API / Backend

**Path:** `apps/api`  
**Stack:** NestJS, TypeScript, Prisma ORM, PostgreSQL (Supabase), GraphQL (Apollo), Stellar SDK

**Module Overview:**
```
src/
├── auth/           # Stellar wallet authentication (SEP-10)
├── patients/       # Patient CRUD and record management
├── records/        # Record encryption, IPFS upload, on-chain anchoring
├── consent/        # Consent grant/revoke + access checking middleware
├── prescriptions/  # Medication management + drug interaction checks
├── billing/        # Bill creation and on-chain claim anchoring
├── labs/           # Lab result management and IPFS linking
├── appointments/   # Appointment scheduling + Soroban token management
├── credentials/    # Healthcare professional credential verification
├── analytics/      # Aggregate anonymized public health data
└── stellar/        # Soroban contract interaction services
```

```bash
cd apps/api
pnpm dev         # http://localhost:4000
pnpm db:migrate
pnpm db:seed
```

### Mobile App

**Path:** `apps/mobile`  
**Stack:** React Native (Expo), TypeScript

**Target Users:** Patients  
**Key Features:**
- View personal health timeline
- Manage consent grants (grant/revoke provider access)
- View prescriptions and vaccination records
- Emergency QR code generation
- Receive appointment reminders
- Securely share records with new providers via QR scan

```bash
cd apps/mobile
pnpm start       # Expo dev server
pnpm android
pnpm ios
```

---

## Packages

### `packages/ui`
Shared component library used across all apps.

```bash
pnpm --filter @vitalisx/ui build
```

Key components: `<RecordCard>`, `<ConsentTimeline>`, `<MedicationList>`, `<PatientBadge>`, `<DrugInteractionAlert>`, `<ConsentGrantModal>`, `<StellarWalletConnect>`, `<EmergencyQRCode>`

### `packages/stellar-sdk-utils`
Abstractions over the Stellar JS SDK and Soroban contract invocations.

```typescript
import { invokeContract, signAndSubmit, buildConsentGrant } from '@vitalisx/stellar-sdk-utils';

const result = await invokeContract({
  contractId: CONSENT_MANAGER_CONTRACT_ID,
  method: 'grant_consent',
  args: [patientId, providerAddress, scope, expiresAt],
  wallet: freighterWallet,
});
```

### `packages/crypto`
Patient-derived key management and record encryption/decryption.

```typescript
import { derivePatientKey, encryptRecord, decryptRecord } from '@vitalisx/crypto';

// Derive a symmetric key from the patient's Stellar keypair
const key = await derivePatientKey(patientStellarPublicKey);

// Encrypt before IPFS upload
const { ciphertext, iv } = await encryptRecord(key, recordPayload);

// Decrypt after retrieval
const plaintext = await decryptRecord(key, ciphertext, iv);
```

### `packages/drug-db`
Embedded, offline-capable drug interaction database (RxNorm-compatible).

```typescript
import { checkInteractions, getDrugInfo } from '@vitalisx/drug-db';

const interactions = await checkInteractions(['warfarin', 'aspirin', 'ibuprofen']);
// Returns: [{ drugs: ['warfarin', 'ibuprofen'], severity: 'HIGH', description: '...' }]
```

### `packages/validators`
Zod schemas for all clinical data structures, shared between frontend and backend.

```typescript
import { DiagnosisSchema, PrescriptionSchema, PatientRegistrationSchema } from '@vitalisx/validators';

const parsed = DiagnosisSchema.parse(formData); // throws on invalid clinical data
```

---

## Getting Started

### Prerequisites

- Node.js >= 20
- pnpm >= 9
- Docker & Docker Compose
- Rust (for Soroban contract development)
- Stellar CLI: `cargo install --locked stellar-cli`
- A [Freighter wallet](https://www.freighter.app/) for local testing

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/vitalisx.git
cd vitalisx

# Install all dependencies
pnpm install

# Start local infrastructure (PostgreSQL, Redis)
docker-compose up -d

# Copy environment files
cp apps/api/.env.example apps/api/.env
cp apps/web/.env.example apps/web/.env
```

### Build & Deploy Soroban Contracts

```bash
# Build all contracts
cd contracts/patient-registry
cargo build --target wasm32-unknown-unknown --release

# Deploy to Stellar Testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/patient_registry.wasm \
  --source-account your-secret-key \
  --network testnet

# Initialize the contract
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account your-secret-key \
  --network testnet \
  -- initialize \
  --admin your-public-key
```

Repeat for all contracts in the `contracts/` directory. Store the resulting contract IDs in your `.env` files.

### Run the Full Stack

```bash
# From the monorepo root
pnpm dev          # Starts all apps in parallel via Turborepo

# Or individually
pnpm --filter @vitalisx/web dev
pnpm --filter @vitalisx/api dev
```

### Run Tests

```bash
pnpm test                          # All packages and apps
pnpm --filter @vitalisx/api test  # API tests only

# Soroban contract tests
cd contracts/consent-manager
cargo test
```

---

## Environment Variables

### `apps/api/.env`

```env
# Database
DATABASE_URL=postgresql://vitalisx:password@localhost:5432/vitalisx

# Stellar
STELLAR_NETWORK=testnet
STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
STELLAR_RPC_URL=https://soroban-testnet.stellar.org

# Contract IDs (from deployment)
PATIENT_REGISTRY_CONTRACT_ID=C...
CONSENT_MANAGER_CONTRACT_ID=C...
RECORD_ANCHOR_CONTRACT_ID=C...
CREDENTIAL_VAULT_CONTRACT_ID=C...
BILLING_LEDGER_CONTRACT_ID=C...
APPOINTMENT_TOKEN_CONTRACT_ID=C...

# IPFS (Filebase or Pinata)
IPFS_GATEWAY=https://ipfs.filebase.io/ipfs
IPFS_API_KEY=your_api_key
IPFS_SECRET=your_secret

# Application
JWT_SECRET=your_jwt_secret
API_PORT=4000
```

### `apps/web/.env.local`

```env
NEXT_PUBLIC_API_URL=http://localhost:4000
NEXT_PUBLIC_STELLAR_NETWORK=testnet
NEXT_PUBLIC_PATIENT_REGISTRY_CONTRACT_ID=C...
NEXT_PUBLIC_CONSENT_MANAGER_CONTRACT_ID=C...
NEXT_PUBLIC_RECORD_ANCHOR_CONTRACT_ID=C...
```

---

## Database Schema

The off-chain PostgreSQL database (managed via Prisma) mirrors and indexes on-chain state for fast querying.

```prisma
model Patient {
  id              String   @id @default(cuid())
  stellarAddress  String   @unique
  patientChainId  BigInt   @unique   // on-chain patient_id
  dateOfBirth     DateTime
  bloodType       String?
  createdAt       DateTime @default(now())

  records         HealthRecord[]
  consents        ConsentGrant[]
  prescriptions   Prescription[]
  appointments    Appointment[]
  bills           Bill[]
}

model HealthRecord {
  id              String     @id @default(cuid())
  patientId       String
  providerId      String
  recordType      RecordType
  ipfsHash        String
  chainRecordId   BigInt     // on-chain record_id for verification
  anchoredAt      DateTime
  metadata        Json?

  patient         Patient    @relation(fields: [patientId], references: [id])
  provider        Provider   @relation(fields: [providerId], references: [id])
}

model ConsentGrant {
  id              String        @id @default(cuid())
  patientId       String
  providerAddress String
  scope           ConsentScope[]
  expiresAt       DateTime
  chainConsentId  BigInt
  revokedAt       DateTime?

  patient         Patient       @relation(fields: [patientId], references: [id])
}

model Provider {
  id              String         @id @default(cuid())
  stellarAddress  String         @unique
  name            String
  role            ProviderRole
  facility        String
  licenseHash     String?        // on-chain credential hash
  licenseExpiry   DateTime?

  records         HealthRecord[]
}

model Prescription {
  id              String   @id @default(cuid())
  patientId       String
  providerId      String
  drugName        String
  dosage          String
  frequency       String
  startDate       DateTime
  endDate         DateTime?
  status          PrescriptionStatus

  patient         Patient  @relation(fields: [patientId], references: [id])
}
```

---

## Consent & Privacy Model

VitalisX implements a **Zero-Trust Access Control** model where **no data access is possible without explicit patient consent on-chain**.

### Consent Scopes

| Scope | Description |
|---|---|
| `FULL` | Access to all record categories |
| `DIAGNOSIS` | Diagnoses and illness history only |
| `MEDICATIONS` | Prescription history only |
| `LABS` | Lab results and imaging only |
| `VITALS` | Vital signs history only |
| `BILLING` | Billing records only |
| `EMERGENCY` | Emergency access (auto-granted by patient QR code) |

### Access Flow

```
Provider requests record
        │
        ▼
consent-manager.check_access()
        │
  ┌─────┴──────┐
  │            │
GRANTED      DENIED
  │            │
  ▼            ▼
Decrypt &   Return 403
return data    │
  │         Log attempt
  ▼         on-chain
Log access
on-chain
```

### Emergency Access

A patient can generate a time-limited (e.g., 2-hour) emergency access token that any provider can scan. This creates a temporary `EMERGENCY` consent on-chain with a hard expiry, after which access is automatically revoked by the contract.

---

## Tokenomics & Incentives

VitalisX uses Stellar's native asset capabilities for incentive alignment:

- **VITX token (custom Stellar asset)** — issued to providers for quality data entry, used for platform governance
- **USDC payments** — direct patient-to-provider micro-payment for consultations, settled on Stellar
- **Data quality staking** — providers stake VITX tokens; inaccurate or fraudulent records slash their stake
- **Research data bounties** — anonymized, aggregated health data can be contributed (with patient opt-in) to research institutions in exchange for USDC rewards paid to participating patients

---

## Roadmap

### Phase 1 — Foundation (Current)
- [x] Monorepo structure and tooling
- [x] Soroban contract scaffolding (patient-registry, consent-manager, record-anchor)
- [ ] Core API (patients, records, consent)
- [ ] Web app: provider dashboard and record entry
- [ ] Testnet deployment

### Phase 2 — Clinical Features
- [ ] Drug interaction checker integration
- [ ] Lab results + IPFS linking
- [ ] Prescription management
- [ ] Mobile app (patient-facing)
- [ ] Emergency QR code system

### Phase 3 — Finance & Credentials
- [ ] Billing ledger contract
- [ ] USDC payment settlement
- [ ] Credential vault for provider licensing
- [ ] Appointment tokenization

### Phase 4 — Network & Analytics
- [ ] Multi-facility network (hospital onboarding)
- [ ] Anonymized public health analytics dashboard
- [ ] VITX token launch and governance
- [ ] Research data bounty marketplace
- [ ] Mainnet deployment

---

## Contributing

We welcome contributions from healthcare professionals, blockchain developers, and anyone who cares about health data sovereignty.

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes following the [Contributing Guide](docs/CONTRIBUTING.md)
4. Ensure all tests pass: `pnpm test`
5. Submit a pull request

See open issues tagged `good first issue` for entry points.

### Development Guidelines

- All Soroban contracts must have 90%+ test coverage
- All API endpoints must include OpenAPI documentation
- Patient data must never be logged in plaintext anywhere in the codebase
- All PRs touching consent logic require a security review

---

## License

MIT — see [LICENSE](LICENSE) for full terms.

---

## Acknowledgements

Built on the Stellar blockchain as an open-source public good for global healthcare infrastructure.

---

*VitalisX — Because your health history should belong to you.*
