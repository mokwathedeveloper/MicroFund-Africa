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

### Running with Docker (Recommended for Demo)
For the easiest setup, use Docker Compose to spin up the entire stack including the database:
1. `docker-compose up --build`
2. Access the frontend at `http://localhost:8081`
3. Access the backend API at `http://localhost:8080/api`

### Running the Backend Locally
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

- [x] **Reputation-Based Credit Laddering**: (Innovation) Dynamic loan limits and Reputation Tiers (Bronze, Silver, Gold).

- [x] **Multi-Platform Support**: (Usability) Swahili i18n and professional Dark Mode.

- [x] **Secure Identity**: (Technical) Argon2 hashing, JWT sessions, and strict DTO validation.

- [x] **P2P Marketplace**: Peer-to-peer lending ecosystem.

- [x] **Mobile Money Integration**: Simulated M-Pesa STK Push.

- [x] **High-Performance WASM**: Yew frontend with offline-first caching.

- [x] **Observability**: Industrial-grade logging with `tracing`.



## Technical Highlights

- **Memory Safety**: Leverages Rust's ownership model across the entire stack.

- **Async Power**: High-concurrency handling using `tokio` and `actix-web`.

- **WASM Performance**: The Yew frontend compiles to highly efficient WebAssembly.

- **Custom Error Handling**: Robust error propagation using `thiserror` and `AppError` middleware.

- [x] **Strict Validation**: Type-safe input validation using the `validator` crate.

- [x] **CI/CD Quality**: Automated `clippy`, `rustfmt`, and build tests via GitHub Actions.

- **Blockchain Simulation**: Automated logging of loan lifecycle events to a simulated ledger.

- **Payment Integration**: Simulated M-Pesa STK Push for seamless deposits and repayments.



## AI Transparency

This project was developed with the assistance of a Large Language Model (LLM) to ensure rapid prototyping, memory-safe Rust implementation, and comprehensive documentation for the Rust Africa Hackathon 2026. All core business logic and architectural choices were guided by the user to address specific African financial challenges.




