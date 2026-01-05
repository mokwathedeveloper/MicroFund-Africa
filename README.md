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
- [x] Microloan submission (Backend + Frontend)
- [x] Loan repayment workflow
- [x] Dashboard with real-time API data
- [x] Solana Smart Contract for Loan logging
- [x] Responsive CSS styling

## Demo Instructions
1. Register a new user on the `/register` page.
2. Login to access the Dashboard.
3. Submit a loan request (e.g., $50 for "Farm Seeds").
4. View the pending loan in your dashboard.
5. Click "Repay" to simulate loan settlement.

*Note: Smart contract interactions are simulated in the backend logs for this demo version to avoid wallet dependency issues during initial setup.*