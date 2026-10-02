# VaultCore IRE — File Plan

## File Tree

```text
vaultcore/
├── cpp/
│   ├── sensor/
│   │   ├── main.cpp
│   │   ├── file_monitor.cpp
│   │   ├── file_monitor.hpp
│   │   ├── event_queue.cpp
│   │   ├── event_queue.hpp
│   │   ├── transport.cpp
│   │   ├── transport.hpp
│   │   ├── crypto.cpp
│   │   ├── crypto.hpp
│   │   ├── service.cpp
│   │   └── service.hpp
│   │
│   ├── ire/
│   │   ├── main.cpp
│   │   ├── server.cpp
│   │   ├── server.hpp
│   │   ├── ingest.cpp
│   │   ├── ingest.hpp
│   │   ├── recovery.cpp
│   │   └── recovery.hpp
│   │
│   ├── gui/
│   │   ├── main.cpp
│   │   ├── dashboard.cpp
│   │   ├── dashboard.hpp
│   │   ├── recovery_view.cpp
│   │   └── recovery_view.hpp
│   │
│   └── shared/
│       ├── event.hpp
│       ├── protocol.hpp
│       ├── config.hpp
│       ├── rust_bridge.cpp
│       └── rust_bridge.hpp
│
├── rust/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── validator.rs
│   │   ├── integrity.rs
│   │   ├── storage.rs
│   │   └── protocol.rs
│   │
│   └── include/
│       └── vaultcore_rust.h
│
└── asm/
    ├── cpu.asm
    ├── timing.asm
    └── memory.asm
```

---

# What Every File Does

## C++ Sensor

### `cpp/sensor/main.cpp`
Starts the VaultCore sensor.

Used for:
- loading config
- starting file monitoring
- starting the event queue
- starting networking
- initializing crypto
- starting the Windows service

### `cpp/sensor/file_monitor.hpp`
Declares the file monitoring interface.

Used for:
- monitor start/stop
- monitored paths
- file event callbacks

### `cpp/sensor/file_monitor.cpp`
Detects file changes on Windows.

Used for:
- create
- write
- rename
- delete
- timestamps
- file size
- building `FileEvent`

### `cpp/sensor/event_queue.hpp`
Declares the event queue.

Used for:
- queue interface
- push/pop
- shutdown

### `cpp/sensor/event_queue.cpp`
Implements the event queue.

Used for:
- storing events before sending
- thread safety
- buffering
- backpressure

### `cpp/sensor/transport.hpp`
Declares sensor-to-IRE networking.

Used for:
- connect
- disconnect
- send event
- send data

### `cpp/sensor/transport.cpp`
Implements networking.

Used for:
- connection to VaultCore IRE
- reconnect
- timeouts
- acknowledgements
- sequence numbers
- TLS/mTLS integration later

### `cpp/sensor/crypto.hpp`
Declares crypto helpers.

Used for:
- hashing
- random generation
- encryption wrappers
- key/certificate handling

### `cpp/sensor/crypto.cpp`
Implements crypto integration.

Used for:
- Windows CNG or other trusted crypto backend
- payload protection
- integrity helpers
- secure random values

### `cpp/sensor/service.hpp`
Declares the Windows service interface.

### `cpp/sensor/service.cpp`
Runs the sensor as a Windows service.

Used for:
- service start
- stop
- shutdown
- Service Control Manager integration

---

# C++ IRE

### `cpp/ire/main.cpp`
Starts the VaultCore IRE server.

Used for:
- initializing server
- initializing Rust bridge
- initializing storage/recovery
- clean shutdown

### `cpp/ire/server.hpp`
Declares the IRE server interface.

### `cpp/ire/server.cpp`
Handles incoming sensor connections.

Used for:
- listening
- authentication
- receiving messages
- rejecting invalid traffic

### `cpp/ire/ingest.hpp`
Declares the ingest pipeline.

### `cpp/ire/ingest.cpp`
Moves received data into validation.

Used for:
- basic checks
- protocol version
- sequence checks
- calling Rust validation
- routing result

### `cpp/ire/recovery.hpp`
Declares recovery functions.

### `cpp/ire/recovery.cpp`
Handles recovery operations.

Used for:
- listing saves
- validating saves before restore
- selecting recovery point
- export/restore

---

# C++ GUI

### `cpp/gui/main.cpp`
Starts the VaultCore GUI.

### `cpp/gui/dashboard.hpp`
Declares the dashboard.

### `cpp/gui/dashboard.cpp`
Shows:
- sensor status
- IRE status
- recent events
- warnings
- recovery point count

### `cpp/gui/recovery_view.hpp`
Declares recovery UI.

### `cpp/gui/recovery_view.cpp`
Shows:
- saves
- validation state
- quarantine state
- restore controls

---

# Shared C++

### `cpp/shared/event.hpp`
Defines the main `FileEvent`.

Used by both sensor and IRE.

Contains data such as:
- sequence
- timestamp
- sensor ID
- event type
- path
- file size
- process ID

### `cpp/shared/protocol.hpp`
Defines the VaultCore network protocol.

Used for:
- message types
- version
- headers
- lengths
- sequence numbers

### `cpp/shared/config.hpp`
Defines shared configuration.

Used for:
- IRE address
- port
- monitored paths
- queue size
- sensor ID
- timeouts

### `cpp/shared/rust_bridge.hpp`
Declares the C++ wrapper around Rust.

### `cpp/shared/rust_bridge.cpp`
Calls Rust through FFI.

Used for:
- validation calls
- integrity calls
- storage logic calls

---

# Rust

### `rust/src/lib.rs`
Main Rust library entry point.

Used for:
- exposing FFI functions
- connecting Rust modules
- converting C data into Rust types

### `rust/src/validator.rs`
Main validation logic.

Used for:
- combining signals
- checking whether incoming data is valid
- returning states such as:
  - VALID
  - SUSPICIOUS
  - INVALID
  - CORRUPT

### `rust/src/integrity.rs`
Checks data integrity.

Used for:
- hashes
- chunk verification
- missing data
- manifest consistency

### `rust/src/storage.rs`
Handles recovery-point logic.

Used for:
- trusted saves
- quarantine
- retention
- save rotation

### `rust/src/protocol.rs`
Parses and validates protocol messages safely.

Used for:
- length checks
- enum validation
- malformed message rejection

### `rust/include/vaultcore_rust.h`
C-compatible header between C++ and Rust.

Used for:
- exported Rust functions
- FFI structs
- return codes

---

# x86-64 ASM

### `asm/cpu.asm`
Optional CPU-specific helpers.

Used for:
- CPU feature checks
- low-level instruction experiments

### `asm/timing.asm`
Optional timing helpers.

Used for:
- profiling
- benchmarks
- low-level timing tests

### `asm/memory.asm`
Optional memory helpers.

Used for:
- memory performance experiments
- comparing compiler output with hand-written ASM

---

# Ownership

## FALSE

FALSE owns the C++ and ASM side.

Files:

```text
cpp/sensor/main.cpp
cpp/sensor/file_monitor.cpp
cpp/sensor/file_monitor.hpp
cpp/sensor/event_queue.cpp
cpp/sensor/event_queue.hpp
cpp/sensor/transport.cpp
cpp/sensor/transport.hpp
cpp/sensor/crypto.cpp
cpp/sensor/crypto.hpp
cpp/sensor/service.cpp
cpp/sensor/service.hpp

cpp/ire/main.cpp
cpp/ire/server.cpp
cpp/ire/server.hpp
cpp/ire/ingest.cpp
cpp/ire/ingest.hpp
cpp/ire/recovery.cpp
cpp/ire/recovery.hpp

cpp/gui/main.cpp
cpp/gui/dashboard.cpp
cpp/gui/dashboard.hpp
cpp/gui/recovery_view.cpp
cpp/gui/recovery_view.hpp

cpp/shared/event.hpp
cpp/shared/protocol.hpp
cpp/shared/config.hpp
cpp/shared/rust_bridge.cpp
cpp/shared/rust_bridge.hpp

asm/cpu.asm
asm/timing.asm
asm/memory.asm
```

FALSE is responsible for:

- Windows sensor
- Windows APIs
- file monitoring
- networking
- crypto integration
- Windows service
- IRE server glue
- GUI
- recovery interface
- C++ ↔ Rust bridge
- optional x86-64 ASM

---

## DENIM

DENIM owns the Rust side.

Files:

```text
rust/src/lib.rs
rust/src/validator.rs
rust/src/integrity.rs
rust/src/storage.rs
rust/src/protocol.rs
rust/include/vaultcore_rust.h
```

DENIM is responsible for:

- validation
- integrity checks
- protocol parsing
- recovery-point logic
- save retention
- quarantine state
- Rust FFI exports

---

# Shared Boundary

The main handoff between FALSE and DENIM is:

```text
FALSE / C++
    ↓
rust_bridge.cpp
    ↓
vaultcore_rust.h
    ↓
DENIM / Rust
```

FALSE sends data into the Rust layer.

DENIM returns simple results back to C++.

Example:

```text
C++ FileEvent
    ↓
Rust validation
    ↓
VALID / SUSPICIOUS / INVALID / CORRUPT
```
