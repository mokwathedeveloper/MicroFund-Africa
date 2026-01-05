.PHONY: build run-backend run-frontend db-init db-seed

# Build both backend and frontend
build:
	cd backend && cargo build
	cd frontend && trunk build

# Start the Actix-web backend
run-backend:
	cd backend && cargo run

# Start the Yew frontend
run-frontend:
	cd frontend && trunk serve --port 8081

# Initialize the database (assumes postgres is running)
db-init:
	sqlx database create
	sqlx migrate run

# Seed the database with demo data
db-seed:
	psql $(DATABASE_URL) -f migrations/seed_data.sql

# Full development environment setup
setup:
	cargo install trunk sqlx-cli
	$(MAKE) db-init
	$(MAKE) db-seed
