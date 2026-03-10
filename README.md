#  Rust RWA Gold Protocol (v1.0)

A high-concurrency, multi-threaded Real-World Asset (RWA) simulation built in Rust. This protocol manages digital gold tokens backed by a physical vault, featuring a live market oracle and an automated yield engine.

##Core Features
- **Live Price Oracle:** A background thread simulating market volatility, updating gold prices in real-time.
- **Automated Yield Engine:** A secondary background thread that distributes compound interest (0.01%/min) to all active INR balances.
- **Thread-Safe State:** Uses `Arc<Mutex<T>>` to allow safe, concurrent access across the Oracle, Yield, and Main threads.
- **Persistent Ledger:** Full transaction history saved to `bank_vault.json` using `Serde`.
- **Audit System:** One-click HTML report generation for transparency and "Proof of Reserve" checks.
- **Role-Based Access:** Distinction between `Admin` (vault management) and `Customer` (trading) roles.

##Architecture
The system consists of three main components interacting with a shared state:
1. **Main Thread:** Handles the CLI User Interface and transaction logic.
2. **Oracle Thread:** Simulates external market data feeds.
3. **Yield Thread:** Simulates DeFi-style liquidity rewards.



##Tech Stack
- **Language:** Rust
- **Math:** `rust_decimal` for precision-safe financial calculations.
- **Time:** `chrono` for UTC transaction stamping.
- **Serialization:** `serde_json` for data persistence.

##How to Run
1. Clone the repo: `git clone <your-repo-link>`
2. Run the project: `cargo run`
3. Export an audit: Select Option `9` to generate `audit_report.html`.
