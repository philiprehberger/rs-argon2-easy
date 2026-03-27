# rs-argon2-easy

[![CI](https://github.com/philiprehberger/rs-argon2-easy/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-argon2-easy/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-argon2-easy.svg)](https://crates.io/crates/philiprehberger-argon2-easy)
[![GitHub release](https://img.shields.io/github/v/release/philiprehberger/rs-argon2-easy)](https://github.com/philiprehberger/rs-argon2-easy/releases)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-argon2-easy)](https://github.com/philiprehberger/rs-argon2-easy/commits/main)
[![License](https://img.shields.io/github/license/philiprehberger/rs-argon2-easy)](LICENSE)
[![Bug Reports](https://img.shields.io/github/issues/philiprehberger/rs-argon2-easy/bug)](https://github.com/philiprehberger/rs-argon2-easy/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![Feature Requests](https://img.shields.io/github/issues/philiprehberger/rs-argon2-easy/enhancement)](https://github.com/philiprehberger/rs-argon2-easy/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

Dead-simple password hashing with Argon2id — `hash()` and `verify()`, nothing more

## Installation

```toml
[dependencies]
philiprehberger-argon2-easy = "0.1.1"
```

## Usage

```rust
use philiprehberger_argon2_easy::{hash, verify};

// Hash a password
let hashed = hash("my-password")?;

// Verify a password
let is_valid = verify("my-password", &hashed)?;
assert!(is_valid);
```

### Profiles

```rust
use philiprehberger_argon2_easy::{hash_with, Profile};

// Fast hashing for interactive login
let hashed = hash_with("password", Profile::Interactive)?;

// Slow hashing for encryption keys
let hashed = hash_with("password", Profile::Sensitive)?;
```

### Check if rehashing is needed

```rust
use philiprehberger_argon2_easy::needs_rehash;

if needs_rehash(&stored_hash)? {
    let new_hash = hash(password)?;
    // Store new_hash
}
```

## API

| Function | Description |
|----------|-------------|
| `hash(password)` | Hash with default OWASP-recommended parameters |
| `hash_with(password, profile)` | Hash with a specific profile |
| `verify(password, hash)` | Verify a password against a hash |
| `needs_rehash(hash)` | Check if hash uses outdated parameters |

## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this package useful, consider giving it a star on GitHub — it helps motivate continued maintenance and development.

[![LinkedIn](https://img.shields.io/badge/Philip%20Rehberger-LinkedIn-0A66C2?logo=linkedin)](https://www.linkedin.com/in/philiprehberger)
[![More packages](https://img.shields.io/badge/more-open%20source%20packages-blue)](https://philiprehberger.com/open-source-packages)

## License

[MIT](LICENSE)
