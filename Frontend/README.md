# VitalisX Frontend

🏥 **VitalisX Frontend — Decentralized Healthcare Records Interface**

VitalisX Frontend is the primary user interface for the VitalisX ecosystem, a privacy-first healthcare records platform built on the Stellar blockchain. Designed for healthcare professionals, patients, hospitals, laboratories, and healthcare administrators, the application provides secure access to medical records, consent management, billing, appointments, and healthcare analytics through a modern web experience.

Built with Next.js 14, TypeScript, Tailwind CSS, and Stellar wallet integration, the frontend delivers a responsive, secure, and intuitive healthcare workflow powered by decentralized infrastructure.

---

# 🚀 Overview

VitalisX Frontend serves as the gateway to decentralized healthcare management.

The application enables users to:

* Access patient health records
* Create and manage clinical records
* Grant and revoke healthcare provider consent
* Manage prescriptions and medications
* Upload and review laboratory results
* Schedule appointments
* Verify healthcare credentials
* View billing and insurance information
* Connect Stellar wallets
* Interact with Soroban-powered healthcare contracts

The frontend communicates with the VitalisX Backend and Stellar blockchain to provide a seamless healthcare experience while preserving patient privacy and ownership.

---

# 🔐 Core Features

## Patient Health Records

* Unified patient health timeline
* Diagnosis history
* Treatment records
* Allergy tracking
* Vaccination history
* Medical record search

## Consent Management

* Grant provider access
* Revoke permissions instantly
* Manage consent scopes
* View access history
* Emergency access generation

## Prescription Management

* Prescription creation
* Medication history
* Drug interaction alerts
* Refill tracking
* Prescription verification

## Laboratory Services

* Upload laboratory results
* View diagnostic reports
* Link imaging records
* IPFS-backed document storage
* Lab result verification

## Billing & Claims

* Medical bill review
* Insurance claim tracking
* Payment history
* Billing transparency dashboard
* Settlement monitoring

## Appointment Management

* Schedule appointments
* Manage bookings
* View provider availability
* Appointment reminders
* Appointment status tracking

## Credential Verification

* Verify healthcare licenses
* Professional credential management
* Facility verification
* Credential status monitoring

---

# ⚙️ Technology Stack

## Frontend Framework

* Next.js 14
* React 18
* TypeScript

## UI & Styling

* Tailwind CSS
* Radix UI
* Lucide Icons
* Framer Motion

## State Management

* React Query / TanStack Query
* Zustand
* React Context

## Authentication

* Stellar Wallet Authentication
* Freighter Wallet
* JWT Session Management

## Forms & Validation

* React Hook Form
* Zod

## Blockchain Integration

* Stellar SDK
* Soroban Client Libraries

---

# 🏗 Frontend Architecture

```text
src/
├── app/
├── components/
├── features/
├── hooks/
├── services/
├── providers/
├── lib/
├── store/
├── types/
├── utils/
└── constants/
```

### Directory Structure

| Folder     | Purpose                         |
| ---------- | ------------------------------- |
| app        | Next.js App Router pages        |
| components | Shared UI components            |
| features   | Feature-specific modules        |
| hooks      | Custom React hooks              |
| services   | API and blockchain integrations |
| providers  | Application providers           |
| store      | Global state management         |
| lib        | Utility libraries               |
| types      | Shared TypeScript definitions   |

---

# 🖥 Key Screens

## Provider Dashboard

```text
/dashboard
```

Features:

* Recent patients
* Pending actions
* Upcoming appointments
* Clinical alerts
* Analytics overview

---

## Patient Registry

```text
/patients
```

Features:

* Patient search
* Patient registration
* Record lookup
* Provider assignment

---

## Patient Timeline

```text
/patients/[id]
```

Features:

* Medical history
* Diagnoses
* Treatments
* Prescriptions
* Laboratory results

---

## New Medical Record

```text
/patients/[id]/records/new
```

Features:

* Diagnosis entry
* Treatment planning
* Prescription creation
* Clinical notes

---

## Consent Portal

```text
/consent
```

Features:

* Grant access
* Revoke access
* View active permissions
* Review audit history

---

## Prescriptions

```text
/prescriptions
```

Features:

* Medication management
* Drug interaction checks
* Prescription history

---

## Laboratory Portal

```text
/labs
```

Features:

* Upload lab results
* View diagnostics
* Manage imaging references

---

## Billing

```text
/billing
```

Features:

* Medical invoices
* Claims tracking
* Payment records

---

## Credentials

```text
/credentials
```

Features:

* License verification
* Provider credential management

---

# 🔗 Stellar & Soroban Integration

The frontend interacts directly with Stellar wallets and Soroban contracts.

Supported integrations:

* Freighter Wallet
* Stellar Testnet
* Stellar Mainnet
* Soroban Smart Contracts

Supported contracts:

* Patient Registry
* Consent Manager
* Record Anchor
* Credential Vault
* Billing Ledger
* Appointment Token

Users sign transactions directly from their wallet before blockchain submission.

---

# 🔒 Privacy & Security

VitalisX follows a patient-first privacy model.

### Security Features

* Wallet-based authentication
* Encrypted healthcare records
* Consent-based access control
* Secure API communication
* Client-side record encryption
* Protected routes
* Session management
* Audit visibility

### Consent Workflow

```text
Patient
   │
   ▼
Grant Consent
   │
   ▼
Wallet Signature
   │
   ▼
Soroban Contract
   │
   ▼
Access Granted
```

Every permission change requires explicit user authorization.

---

# 📦 Installation

## Requirements

* Node.js v20+
* pnpm v9+
* Freighter Wallet Extension

## Clone Repository

```bash
git clone https://github.com/your-org/vitalisx.git
cd apps/web
```

## Install Dependencies

```bash
pnpm install
```

## Configure Environment

```bash
cp .env.example .env.local
```

## Start Development Server

```bash
pnpm dev
```

Application runs at:

```text
http://localhost:3000
```

---

# ⚙️ Environment Variables

Create a `.env.local` file:

```env
NEXT_PUBLIC_API_URL=http://localhost:4000

NEXT_PUBLIC_STELLAR_NETWORK=testnet

NEXT_PUBLIC_PATIENT_REGISTRY_CONTRACT_ID=C...
NEXT_PUBLIC_CONSENT_MANAGER_CONTRACT_ID=C...
NEXT_PUBLIC_RECORD_ANCHOR_CONTRACT_ID=C...
NEXT_PUBLIC_CREDENTIAL_VAULT_CONTRACT_ID=C...
NEXT_PUBLIC_BILLING_LEDGER_CONTRACT_ID=C...
NEXT_PUBLIC_APPOINTMENT_TOKEN_CONTRACT_ID=C...
```

---

# 🧩 Core Components

VitalisX Frontend includes reusable healthcare-specific components.

### Patient Components

* PatientCard
* PatientProfile
* HealthTimeline
* MedicalHistoryViewer

### Consent Components

* ConsentGrantModal
* ConsentTimeline
* ConsentManager

### Prescription Components

* MedicationList
* DrugInteractionAlert
* PrescriptionForm

### Clinical Components

* DiagnosisCard
* TreatmentCard
* LabResultViewer
* VaccinationLedger

### Blockchain Components

* StellarWalletConnect
* TransactionStatus
* ContractInteractionModal

---

# 🧪 Testing

Run tests:

```bash
pnpm test
```

Run linting:

```bash
pnpm lint
```

Build production version:

```bash
pnpm build
```

Run production build:

```bash
pnpm start
```

---

# ♿ Accessibility

VitalisX Frontend prioritizes accessibility for healthcare environments.

Features include:

* Keyboard navigation
* Screen reader support
* WCAG-compliant interfaces
* High-contrast mode compatibility
* Responsive design
* Mobile and tablet optimization

---

# 🚀 Deployment

Supported deployment providers:

* Vercel
* Netlify
* AWS Amplify
* Cloudflare Pages

Recommended production setup:

* Frontend: Vercel
* Backend: NestJS API
* Database: PostgreSQL
* Blockchain: Stellar Mainnet
* Storage: IPFS/Filebase

---

# 🛣 Roadmap

### Phase 1

* Provider dashboard
* Patient registry
* Record management
* Consent portal
* Testnet deployment

### Phase 2

* Drug interaction checker
* Lab management
* Advanced patient timeline
* Emergency QR access

### Phase 3

* Billing portal
* Credential verification
* Appointment tokenization
* Mobile responsiveness improvements

### Phase 4

* Multi-facility dashboards
* Public health analytics
* Governance integration
* Mainnet launch

---

# 🤝 Contributing

We welcome contributions from:

* Frontend developers
* Healthcare professionals
* UX/UI designers
* Accessibility advocates
* Stellar ecosystem contributors

Steps:

1. Fork the repository

```bash
git checkout -b feat/your-feature
```

2. Make changes
3. Run tests and lint checks
4. Submit a pull request

---

# 📄 License

MIT License

---

## VitalisX Frontend

Empowering healthcare professionals and patients with secure, decentralized, and patient-owned healthcare experiences on Stellar.
