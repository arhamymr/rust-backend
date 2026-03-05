# Rust Backend (Actix Web + SeaORM + PASETO)

A production-ready Rust backend template featuring Actix Web, SeaORM, and PASETO-based authentication. This project provides secure access/refresh token flows, cookie-based refresh handling, structured error utilities, and a clean modular layout.

- Web framework: Actix Web 4
- ORM: SeaORM (PostgreSQL via SQLx)
- Auth: PASETO v4.public (Ed25519) access tokens + v4.local (symmetric) refresh tokens
- Logging: env_logger with structured middleware
- Validation: validator
- Password hashing: argon2


## Features

- Register, login, refresh access token, logout, and logout-all endpoints
- Refresh token stored in HTTP-only cookies with configurable attributes
- Access token signed with Ed25519; refresh token encrypted symmetrically
- Clean separation between handlers, services, entities, middlewares, and utils
- Configurable via .env with sensible development defaults
- Integration and unit tests for auth handlers


## Requirements

- Rust toolchain (installed via rustup)
- PostgreSQL (recommended), or modify DB config to your target database
- OpenSSL, Node.js, or Python for key generation (optional; dev can auto-generate)


## Getting Started

1) Clone and prepare environment

```bash
# Linux/macOS
git clone <your-repo-url>
cd rust-backend
cp .env.example .env

# Windows (PowerShell)
git clone <your-repo-url>
cd rust-backend
copy .env.example .env
```

2) Configure environment variables

Open .env and set the values. See the full list below. In dev, you may leave keys empty and set DEV_FALLBACK_KEYS=true to auto-generate ephemeral keys (not for production).

3) Run the server

```bash
# from project root
cargo run
```

By default the server binds to 127.0.0.1:4444. Change via SERVER_PORT in .env.

4) Run tests

```bash
cargo test
```


## Environment Variables

All environment variables live in .env. Reference: .env.example

- ACCESS_PRIVATE_KEY_BASE64: Base64 of raw Ed25519 private key seed (32 bytes)
- ACCESS_PUBLIC_KEY_BASE64: Base64 of raw Ed25519 public key (32 bytes)
- REFRESH_SYMMETRIC_KEY_BASE64: Base64 of 32 random bytes for v4.local
- TOKEN_ISS: Token issuer claim (e.g., apsara-devkit)
- TOKEN_AUD: Token audience claim (e.g., web)
- ACCESS_TOKEN_TTL_MIN: Access token TTL in minutes (e.g., 15)
- REFRESH_TOKEN_TTL_DAYS: Refresh token TTL in days (e.g., 7)
- COOKIE_SECURE: true/false; set true in production (HTTPS only)
- COOKIE_DOMAIN: Cookie domain (e.g., localhost)
- COOKIE_PATH: Cookie path (e.g., /)
- REFRESH_COOKIE_NAME: Name of the refresh cookie (e.g., refresh_token)
- CORS_ALLOWED_ORIGIN: Allowed origin for CORS (e.g., http://localhost:1111)
- SERVER_PORT: Server port (default 4444)
- DEV_FALLBACK_KEYS: true/false; dev-only fallback to ephemeral keys when unset


## Key Generation

Development: You can set DEV_FALLBACK_KEYS=true and leave keys empty to run quickly. For production, generate and store keys securely.

- Generate Ed25519 keys using Node.js (tweetnacl):

```bash
# one-time
npm install tweetnacl

# generate base64 raw keys
node -e "const nacl=require('tweetnacl');const kp=nacl.sign.keyPair();console.log('ACCESS_PRIVATE_KEY_BASE64='+Buffer.from(kp.secretKey.slice(0,32)).toString('base64'));console.log('ACCESS_PUBLIC_KEY_BASE64='+Buffer.from(kp.publicKey).toString('base64'))"
```

- Generate refresh symmetric key (32 bytes) on Windows PowerShell:

```powershell
$bytes = New-Object 'System.Byte[]' 32; 
[System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes); 
[Convert]::ToBase64String($bytes)
```

- Generate refresh symmetric key (32 bytes) on Linux/macOS:

```bash
openssl rand -base64 32
```

Paste results into .env.


## Project Structure

```
src/
  main.rs                # Actix setup: routes, middleware, server
  db/
    connection.rs        # SeaORM DB connection
  entities/              # SeaORM models and prelude
  handlers/              # Actix route handlers
    auth/                # Auth endpoints + tests
    users.rs             # User profile endpoints
  middlewares/           # Custom middlewares
  services/              # Business logic
  utils/                 # Errors, hashing, paseto helpers
.env                      # Your environment variables
.env.example              # Example environment
Cargo.toml                # Dependencies and metadata
```


## Running and Configuration

- Logging: controlled via RUST_LOG or default filter in code (env_logger). Example:

```bash
RUST_LOG=info cargo run
```

- CORS: Set CORS_ALLOWED_ORIGIN to your frontend origin.
- Cookies: Ensure COOKIE_SECURE=true in production (HTTPS) and set proper domain/path.


## API Reference

Base URL: http://127.0.0.1:4444

Auth

- POST /api/v1/auth/register
  - Body (JSON):
    ```json
    {
      "email": "john@example.com",
      "username": "john_doe",
      "password": "StrongP@ssw0rd",
      "name": "John Doe"
    }
    ```
  - Response (JSON): user summary or success status

- POST /api/v1/auth/login
  - Body (JSON):
    ```json
    {
      "email": "john@example.com",
      "password": "StrongP@ssw0rd"
    }
    ```
  - Response (JSON):
    ```json
    {
      "access_token": "<PASETO v4.public>",
      "token_type": "Bearer",
      "expires_in": 900
    }
    ```
    Refresh token is set as HTTP-only cookie named REFRESH_COOKIE_NAME.

- POST /api/v1/auth/refresh
  - Requires refresh cookie. Returns a fresh access token.

- POST /api/v1/auth/logout
  - Invalidates the current refresh token (cookie cleared).

- POST /api/v1/auth/logout_all
  - Invalidates all active refresh tokens for the user.

Users

- GET /api/v1/users/me
  - Returns current user info using the access token.

- PUT /api/v1/users/me
  - Body (JSON):
    ```json
    {
      "name": "John Doe Updated"
    }
    ```
  - Response (JSON): updated user info


## Example Requests (Windows PowerShell)

```powershell
# Register
curl -X POST "http://127.0.0.1:4444/api/v1/auth/register" `
  -H "Content-Type: application/json" `
  -d "{\"email\":\"john@example.com\",\"username\":\"john_doe\",\"password\":\"StrongP@ssw0rd\",\"name\":\"John Doe\"}"

# Login
curl -X POST "http://127.0.0.1:4444/api/v1/auth/login" `
  -H "Content-Type: application/json" `
  -d "{\"email\":\"john@example.com\",\"password\":\"StrongP@ssw0rd\"}"

# Use access token
$ACCESS="<paste-access-token>"
curl -H "Authorization: Bearer $ACCESS" "http://127.0.0.1:4444/api/v1/users/me"

# Update profile
curl -X PUT "http://127.0.0.1:4444/api/v1/users/me" `
  -H "Authorization: Bearer $ACCESS" `
  -H "Content-Type: application/json" `
  -d "{\"name\":\"John Doe Updated\"}"
```


## Implementation Notes

- The Actix application sets up compression and two logger middlewares to capture remote address and User-Agent.
- Database connection is established via SeaORM in db/connection.rs; adjust to your DB and credentials.
- PASETO helpers and key handling are in utils/paseto.rs; ensure base64 raw bytes are provided.
- Error handling lives in utils/errors.rs using thiserror.
- Password hashing is implemented via argon2 with password-hash traits.
- Auth handlers and request models live under handlers/auth.


## Production Hardening Checklist

- Provide real Ed25519 keys and refresh symmetric key via secrets manager
- Set COOKIE_SECURE=true and run behind HTTPS
- Set strong CORS_ALLOWED_ORIGIN to your exact frontend domain
- Use a persistent database with migrations and indexes
- Configure RUST_LOG=info or warn; aggregate logs centrally
- Review token TTLs (short access, longer refresh)
- Enable database connection pooling and timeouts


## Troubleshooting

- Server fails to start: verify DB connectivity in db/connection.rs and any required env vars.
- 401 Unauthorized: ensure Authorization header is set with Bearer access token.
- Refresh fails: confirm cookie is present and not blocked by browser policy; check domain/path/secure.
- CORS errors: update CORS_ALLOWED_ORIGIN in .env to match your frontend.


## License

Unlicensed. Add your license of choice (MIT/Apache-2.0) before production.
