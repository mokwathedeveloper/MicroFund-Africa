# MicroFund Africa

Microloan and savings platform for unbanked Africans. Built with Rust.

## Architecture

- **Backend**: Actix-web REST API with PostgreSQL (SQLx). Handles user management, loan processing, and integrates with the blockchain.
- **Smart Contracts**: Solana (Anchor) contracts for transparent and secure loan logging and P2P lending.
- **Frontend**: Yew (WASM) for a high-performance, safe, and responsive web interface.

## Project Structure

- `/backend`: Rust Actix-web server.
- `/frontend`: Yew WebAssembly frontend.
- `/contracts`: Solana smart contracts.

## Setup Instructions

### Prerequisites
- Rust (latest stable)
- Node.js & npm (for Trunk/Frontend bundling)
- PostgreSQL
- Solana CLI & Anchor (for contracts)

### Running the Backend
1. `cd backend`
2. Create a `.env` file with `DATABASE_URL=postgres://user:password@localhost/microfund`
3. `cargo run`

### Running the Frontend
1. `cd frontend`
2. `trunk serve`

### Deploying Contracts
1. `cd contracts`
2. `anchor build`
3. `anchor deploy`

## Roadmap
- [ ] User authentication
- [ ] Microloan submission & approval workflow
- [ ] P2P Lending Smart Contract
- [ ] Responsive Dashboard UI
- [ ] Offline-first local storage
