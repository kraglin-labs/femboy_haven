# VaultCore IRE

**VaultCore IRE** is a ransomware-resilient backup and recovery platform built around an **Isolated Recovery Environment (IRE)**.

The system uses lightweight sensors on protected Windows clients and servers to observe file changes and securely forward relevant events and data to an isolated VaultCore server. The IRE performs validation, integrity checks, behavioral analysis, immutable storage, and recovery.

The goal is simple:

> Keep trusted recovery data isolated, validated, and recoverable even if the production environment is compromised.

---

## Architecture

```text
┌────────────────────────── Production Environment ──────────────────────────┐
│                                                                            │
│   Windows Clients                  Windows Servers                         │
│   ┌────────────────┐               ┌────────────────┐                      │
│   │ VaultCore      │               │ VaultCore      │                      │
│   │ Sensor         │               │ Sensor         │                      │
│   └───────┬────────┘               └───────┬────────┘                      │
│           │                                │                               │
│           └──────── File events / data ────┘                               │
│                            │                                               │
└────────────────────────────┼───────────────────────────────────────────────┘
                             │
                             │ End-to-end encrypted
                             │ authenticated channel
                             ▼
┌──────────────────────── VaultCore IRE ─────────────────────────────────────┐
│                                                                            │
│   Ingest ──> Validation ──> Detection ──> Immutable Recovery Store         │
│                  │              │                    │                      │
│                  │              │                    └── Recovery Console   │
│                  │              └── Alerts                                 │
│                  └── Integrity / corruption checks                         │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## VaultCore Sensor

Each protected client and server runs a lightweight VaultCore sensor.

The sensor is intentionally small and does **not** run local AI or heavy analysis.

Its primary responsibilities are:

- monitor important file changes,
- collect relevant file and process metadata,
- queue events during short connectivity loss,
- package file or block changes when required,
- encrypt data before transmission,
- authenticate to the VaultCore IRE,
- report basic sensor health.

Heavy detection and validation are performed centrally inside the IRE.

---

## End-to-End Transport

Communication between the sensor and VaultCore IRE is encrypted and authenticated.

The design should support:

- TLS 1.3 / mutual TLS,
- unique device identities,
- per-sensor certificates or keys,
- encrypted application payloads,
- authenticated encryption,
- replay protection,
- sequence numbers,
- key and certificate rotation,
- sensor revocation.

A compromised sensor identity must be revocable without affecting the rest of the environment.

---

## Recovery Point Validation

A backup is not considered trusted simply because it was received successfully.

Every recovery point moves through a validation pipeline:

```text
RECEIVING
    ↓
VERIFYING
    ↓
ANALYZING
    ↓
VALIDATED
    ↓
IMMUTABLE
```

Suspicious or damaged data is instead moved to:

```text
QUARANTINED
or
REJECTED
```

Validation can include:

- cryptographic integrity checks,
- file and chunk completeness,
- metadata consistency,
- missing or reordered events,
- unusual file entropy changes,
- rapid modification rates,
- mass rename/delete/write activity,
- suspicious process context,
- comparison against previous known-good recovery points.

No single signal should decide whether a recovery point is clean.

---

## Recovery Storage

VaultCore keeps a controlled set of validated recovery points.

The initial design uses **up to five trusted recovery points**.

Important rule:

> A new recovery point must be validated before it is allowed to replace an older trusted recovery point.

This prevents suspicious or corrupted data from automatically pushing the last known-good backup out of the retention window.

Validated recovery points should be stored in an immutable or strongly write-protected form.

---

## Isolated Recovery Environment

The VaultCore server runs inside a separate recovery security boundary.

The IRE should have:

- separate administrative identities,
- no dependency on production Active Directory credentials,
- restricted network exposure,
- no normal SMB access from production,
- no normal RDP/SSH access from production,
- dedicated management access,
- MFA for sensitive recovery operations,
- audit logging,
- controlled export of recovered data.

The production environment should not be able to directly modify trusted recovery points.

---

## Technology Stack

### C++

C++ is the main systems language for VaultCore.

Planned responsibilities:

- Windows sensor,
- file monitoring,
- Windows API integration,
- process and filesystem metadata,
- event queues,
- secure transport client,
- recovery tooling,
- performance-sensitive platform code.

Example areas:

```text
Win32
ETW
USN Journal
Windows Services
Windows CNG
Filesystem APIs
Networking
Concurrency
RAII
```

### Rust

Rust is used where memory safety and reliable concurrent processing are especially valuable.

Planned responsibilities:

- IRE ingest pipeline,
- validation services,
- event processing,
- protocol parsing,
- storage services,
- integrity verification,
- concurrent backend components.

Rust and C++ can communicate through a small, well-defined FFI boundary where required.

### x86-64 Assembly

Assembly is used only where there is a measurable low-level requirement.

Possible uses:

- inspecting generated machine code,
- ABI understanding,
- CPU-specific instructions,
- performance experiments,
- small optimized primitives where compiler output is insufficient.

ASM is **not** used as a security mechanism and is not used simply to make code harder to read.

Preferred order:

```text
C++ / Rust
    ↓
Compiler intrinsics
    ↓
External x86-64 ASM only when justified
```

---

## Core Components

```text
vaultcore/
├── sensor/
│   ├── file-monitor/
│   ├── event-queue/
│   ├── transport/
│   └── health/
│
├── ire/
│   ├── ingest/
│   ├── validator/
│   ├── detection/
│   ├── storage/
│   └── recovery/
│
├── protocol/
│   ├── messages/
│   ├── crypto/
│   └── versioning/
│
├── shared/
│   ├── types/
│   ├── logging/
│   └── config/
│
└── docs/
```

---

## Detection Philosophy

VaultCore should not depend on one detection trick.

For example, high file entropy alone does not mean ransomware. Legitimate encrypted files, archives, databases, and compressed data may also have high entropy.

VaultCore should instead correlate multiple signals over time.

Example:

```text
high write rate
      +
many renamed files
      +
large entropy increase
      +
single process touching many files
      +
unusual delete activity
      ↓
higher risk score
```

The IRE remains responsible for deciding whether incoming data should become a trusted recovery point.

---

## Recovery Flow

```text
Incident detected
      ↓
Production system isolated
      ↓
Administrator opens VaultCore IRE
      ↓
Select known-good recovery point
      ↓
Verify integrity
      ↓
Export / restore to clean system
```

Recovery must not depend on the compromised production environment remaining operational.

---

## Project Goals

- lightweight endpoint sensor,
- minimal impact on clients and servers,
- centralized analysis,
- end-to-end encrypted transport,
- isolated recovery infrastructure,
- validated recovery points,
- immutable storage,
- reliable restore workflow,
- clear audit trail,
- simple and predictable architecture.

---

## Status

VaultCore IRE is currently in the architecture and prototyping phase.

The first implementation targets:

1. basic Windows file-change sensor,
2. authenticated sensor-to-IRE transport,
3. event ingestion,
4. integrity validation,
5. recovery-point storage,
6. simple recovery workflow.

More advanced detection and storage features can be added after the core pipeline is stable.
