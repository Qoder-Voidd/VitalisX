# VitalisX Backend

🏥 **VitalisX Backend — Decentralized Healthcare Records API**

The VitalisX Backend is the core server infrastructure powering the VitalisX healthcare ecosystem. Built with NestJS, PostgreSQL, Redis, and Stellar Soroban integrations, it provides secure APIs, consent enforcement, healthcare record management, credential verification, billing services, appointment scheduling, and blockchain interactions for decentralized patient health records.

This backend acts as the trusted orchestration layer between healthcare providers, patients, encrypted storage systems, and the Stellar blockchain while maintaining strict privacy, compliance, and auditability requirements.

---

# 🚀 Overview

VitalisX Backend enables healthcare institutions and professionals to securely manage patient records while preserving patient ownership and consent.

The backend is responsible for:

* Patient registration and management
* Health record creation and retrieval
* Consent verification and access control
* Healthcare provider credential validation
* Prescription and medication management
* Laboratory result processing
* Medical billing and insurance claim anchoring
* Appointment scheduling
* Blockchain interactions with Stellar Soroban contracts
* IPFS storage integration
* Authentication and authorization
* Audit logging and compliance tracking

---

# 🔐 Core Backend Features

## Patient Management

* Patient registration and profile management
* Patient identity linking with Stellar accounts
* Medical history indexing
* Patient search and retrieval
* Health timeline aggregation

## Health Records

* Structured diagnosis and treatment records
* Encrypted medical record storage
* Record versioning and audit trails
* Record hash anchoring on Stellar
* Medical history retrieval

## Consent Management

* Patient-controlled access permissions
* Consent grant and revocation APIs
* Role-based consent validation
* Time-bound access control
* Consent audit logging

## Provider Credential Verification

* Healthcare professional onboarding
* Credential validation through Soroban contracts
* Facility affiliation management
* License verification services
* Credential expiration monitoring

## Prescription Services

* Prescription management
* Medication history tracking
* Drug interaction checks
* Duplicate prescription detection
* Pharmacy integration support

## Laboratory Services

* Lab result management
* Diagnostic report linking
* Imaging reference storage
* IPFS document anchoring
* Result verification

## Billing & Claims

* Medical bill generation
* Insurance claim anchoring
* Payment settlement tracking
* Billing audit records
* Dispute management support

## Appointment Management

* Appointment scheduling
* Provider availability management
* Appointment reminders
* Appointment token integration
* Scheduling audit logs

## Analytics & Reporting

* Aggregated health metrics
* Facility reporting
* Consent activity analytics
* Clinical data insights
* Public health trend aggregation

---

# ⚙️ Technology Stack

## Backend Framework

* NestJS
* TypeScript
* Node.js 20+

## Database

* PostgreSQL
* Prisma ORM

## Cache & Messaging

* Redis
* WebSocket Gateway

## Blockchain

* Stellar SDK
* Horizon API
* Soroban Smart Contracts

## Storage

* IPFS
* Filebase / Pinata

## Security

* JWT Authentication
* Role-Based Access Control (RBAC)
* Stellar Wallet Authentication
* Record Encryption Services

## Infrastructure

* Docker
* Docker Compose
* AWS / Railway / Render
* GitHub Actions CI/CD

---

# 🏗 Backend Architecture

The VitalisX Backend follows a modular architecture:

```text
src/
├── auth/
├── patients/
├── records/
├── consent/
├── prescriptions/
├── billing/
├── labs/
├── appointments/
├── credentials/
├── analytics/
├── notifications/
├── stellar/
├── ipfs/
├── database/
├── common/
└── config/
```

### Module Responsibilities

| Module        | Purpose                                   |
| ------------- | ----------------------------------------- |
| auth          | Authentication and authorization          |
| patients      | Patient lifecycle management              |
| records       | Medical record operations                 |
| consent       | Consent enforcement and auditing          |
| prescriptions | Medication and prescription management    |
| billing       | Medical billing and claims                |
| labs          | Lab result management                     |
| appointments  | Scheduling and appointment workflows      |
| credentials   | Healthcare provider verification          |
| analytics     | Reporting and public health insights      |
| stellar       | Soroban smart contract interactions       |
| ipfs          | File encryption and decentralized storage |

---

# 🔗 Stellar Integration

The backend communicates with the following Soroban contracts:

## Patient Registry

* Register patient identities
* Manage patient metadata references
* Validate patient ownership

## Consent Manager

* Grant consent
* Revoke consent
* Verify access rights
* Retrieve consent logs

## Record Anchor

* Anchor medical record hashes
* Verify record integrity
* Track record history

## Credential Vault

* Verify healthcare licenses
* Manage credential lifecycle
* Validate provider permissions

## Billing Ledger

* Create billing entries
* Anchor insurance claims
* Track settlements

## Appointment Token

* Manage appointment reservations
* Verify tokenized appointment slots

---

# 🔐 Authentication & Authorization

VitalisX Backend supports:

### Stellar Wallet Authentication

* Freighter Wallet
* Stellar SEP-based authentication
* Wallet signature verification

### JWT Authentication

* Access tokens
* Refresh tokens
* Session management

### Role-Based Access Control

Supported roles:

* PATIENT
* DOCTOR
* NURSE
* PHARMACIST
* LAB_TECHNICIAN
* SPECIALIST
* ADMIN
* HOSPITAL_ADMIN

---

# 🛡 Privacy & Security

Patient data protection is a core requirement.

### Security Measures

* End-to-end encrypted medical records
* Zero-trust access model
* On-chain consent validation
* Immutable audit trails
* Secure API authentication
* Role-based authorization
* IPFS content integrity verification
* Sensitive data redaction from logs

### Access Workflow

```text
Provider Request
        │
        ▼
Authentication
        │
        ▼
Credential Verification
        │
        ▼
Consent Validation
        │
 ┌──────┴───────┐
 │              │
Granted      Denied
 │              │
 ▼              ▼
Decrypt      Return 403
Record
 │
 ▼
Log Access
```

---

# 📦 Installation

## Requirements

* Node.js v20+
* PostgreSQL
* Redis
* Docker
* pnpm

## Clone Repository

```bash
git clone https://github.com/your-org/vitalisx-backend.git
cd vitalisx-backend
```

## Install Dependencies

```bash
pnpm install
```

## Start Infrastructure

```bash
docker-compose up -d
```

## Configure Environment

```bash
cp .env.example .env
```

## Run Database Migrations

```bash
pnpm prisma migrate dev
```

## Seed Database

```bash
pnpm db:seed
```

---

# 🔐 Secrets Management

VitalisX Backend uses HashiCorp Vault for secure secret storage.

## Local Development

```bash
vault server -dev
```

```bash
export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=devroot
```

Provision development secrets:

```bash
./scripts/vault/provision-dev.sh
```

Never commit production secrets to the repository.

---

# ▶ Running the Backend

Development:

```bash
pnpm start:dev
```

Production:

```bash
pnpm build
pnpm start:prod
```

Server:

```text
http://localhost:4000
```

---

# 🧪 Testing

Run unit tests:

```bash
pnpm test
```

Run e2e tests:

```bash
pnpm test:e2e
```

Generate coverage:

```bash
pnpm test:cov
```

---

# 🗄 Database & Migrations

VitalisX Backend uses PostgreSQL and Prisma.

### Generate Migration

```bash
pnpm prisma migrate dev --name migration_name
```

### Apply Migrations

```bash
pnpm prisma migrate deploy
```

### Reset Database

```bash
pnpm prisma migrate reset
```

---

# 🌍 API Capabilities

### Patient APIs

* Create patient
* Update patient
* Search patients
* Retrieve health timeline

### Record APIs

* Create diagnosis records
* Upload treatment records
* Retrieve medical history
* Verify record integrity

### Consent APIs

* Grant consent
* Revoke consent
* View consent logs

### Prescription APIs

* Create prescriptions
* Check drug interactions
* View medication history

### Billing APIs

* Create bills
* Anchor claims
* Track payments

### Appointment APIs

* Create appointments
* Manage schedules
* Confirm attendance

---

# 🚀 Deployment

Supported deployment platforms:

* AWS
* Railway
* Render
* DigitalOcean
* Kubernetes

Recommended production services:

* PostgreSQL
* Redis
* IPFS Provider
* Stellar Testnet/Mainnet
* HashiCorp Vault

---

# 🤝 Contributing

1. Fork the repository
2. Create a feature branch

```bash
git checkout -b feat/feature-name
```

3. Commit changes
4. Sync with latest main branch
5. Submit a pull request

---

# 📜 Development Standards

* All endpoints must include OpenAPI documentation
* All consent operations require security review
* No patient data may be logged in plaintext
* All blockchain interactions must be audited
* Critical modules require automated testing

---

# 📄 License

MIT License

---

## VitalisX Backend

Secure. Auditable. Patient-Owned Healthcare Infrastructure Powered by Stellar.
