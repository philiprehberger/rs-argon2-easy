# rs-argon2-easy

[![CI](https://github.com/philiprehberger/rs-argon2-easy/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-argon2-easy/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-argon2-easy.svg)](https://crates.io/crates/philiprehberger-argon2-easy)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-argon2-easy)](https://github.com/philiprehberger/rs-argon2-easy/commits/main)

Dead-simple password hashing with Argon2id — `hash()` and `verify()`, nothing more

## Installation

```toml
[dependencies]
philiprehberger-argon2-easy = "0.2.0"
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

### Constant-time comparison

```rust
use philiprehberger_argon2_easy::timing_safe_eq;

// Compare tokens or hashes safely without timing leaks
let is_equal = timing_safe_eq("token-a", "token-b");
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
| `timing_safe_eq(a, b)` | Constant-time string comparison |

## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this project useful:

⭐ [Star the repo](https://github.com/philiprehberger/rs-argon2-easy)

🐛 [Report issues](https://github.com/philiprehberger/rs-argon2-easy/issues?q=is%3Aissue+is%3Aopen+label%3Abug)

💡 [Suggest features](https://github.com/philiprehberger/rs-argon2-easy/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)

❤️ [Sponsor development](https://github.com/sponsors/philiprehberger)

🌐 [All Open Source Projects](https://philiprehberger.com/open-source-packages)

💻 [GitHub Profile](https://github.com/philiprehberger)

🔗 [LinkedIn Profile](https://www.linkedin.com/in/philiprehberger)

## License

[MIT](LICENSE)
