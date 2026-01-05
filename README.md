# MicroFund Africa

Microloan and savings platform for unbanked Africans. Built with Rust.

## Problem Statement
Traditional banking systems often exclude millions of Africans due to high fees, lack of physical infrastructure, and strict documentation requirements. **MicroFund Africa** leverages Rust and Web3 technologies to provide a high-performance, secure, and transparent micro-lending solution accessible from any device.

## Architecture

```mermaid
graph TD
    A[Yew WASM Frontend] -->|REST API| B[Actix-web Backend]
    B -->|SQLx| C[(PostgreSQL)]
    B -->|Anchor/Solana| D[Solana Devnet]
    A -->|LocalStorage| E[Offline Cache]
```

- **Backend**: Actix-web REST API with PostgreSQL (SQLx). Handles user management, loan processing, and integrates with the blockchain.
- **Smart Contracts**: Solana (Anchor) contracts for transparent and secure loan logging and P2P lending.
- **Frontend**: Yew (WASM) for a high-performance, safe, and responsive web interface.

## Project Structure

- `/backend`: Rust Actix-web server (Port 8080).
- `/frontend`: Yew WebAssembly frontend (Port 8081).
- `/contracts`: Solana smart contracts.

## Setup Instructions

### Prerequisites
- Rust (latest stable)
- Node.js & npm
- Trunk (`cargo install trunk`)
- PostgreSQL
- Solana CLI & Anchor

### Running the Backend
1. `cd backend`
2. Create a `.env` file:
   ```env
   DATABASE_URL=postgres://user:password@localhost/microfund
   JWT_SECRET=your_super_secret_key
   ```
3. Run migrations (using `sqlx-cli`): `sqlx migrate run`
4. `cargo run`

### Running the Frontend
1. `cd frontend`
2. `trunk serve --port 8081`

### Deploying Contracts
1. `cd contracts`
2. `anchor build`
3. `anchor deploy`

## Implementation Status
- [x] User registration & Login (Argon2 + JWT)
- [x] **Reputation System**: "Trust Score" that increases with every successful repayment.
- [x] Microloan submission (Backend + Frontend)
- [x] **P2P Marketplace**: Peer-to-peer lending where users fund each other's loans.
- [x] **Savings Platform**: Create goals and deposit funds.
- [x] **Offline-First**: LocalStorage caching for loans and savings.
- [x] **Dashboard Analytics**: Real-time summary of borrowing and savings.
- [x] Loan repayment workflow
- [x] Solana Smart Contract for Loan logging
- [x] Responsive CSS styling

## Demo Instructions
1. Register a new user on the `/register` page.
2. Login to access the Dashboard.
3. **Savings**: Create a goal like "Emergency Fund" and deposit some money.
4. **Loans**: Submit a loan request (e.g., $50 for "Farm Seeds").
5. **Analytics**: Observe the summary bar updating in real-time.
6. **Offline**: Refresh the page without an internet connection (if running locally) to see cached data.

*Note: Smart contract interactions are simulated in the backend via the `BlockchainService`. In a live production environment, this module would be replaced with the `solana-client` crate to communicate with the Solana RPC.*



## Technical Highlights

- **Memory Safety**: Leverages Rust's ownership model across the entire stack.

- **Async Power**: High-concurrency handling using `tokio` and `actix-web`.

- **WASM Performance**: The Yew frontend compiles to highly efficient WebAssembly.

- **Custom Error Handling**: Robust error propagation using `thiserror` and `AppError` middleware.

- **Blockchain Simulation**: Automated logging of loan lifecycle events to a simulated ledger.
